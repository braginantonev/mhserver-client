use {
    std::{
        future::Future,
        sync::{
            atomic::{AtomicUsize, Ordering::Relaxed},
            Arc 
        },
    },
    tokio::{
        sync::{
            mpsc::{Receiver, Sender},
            RwLock, mpsc, Mutex
        },
        task::JoinHandle,
    }
};

// non thread-safe
struct CycleArray<T> {
    inner: Vec<T>,
    idx: AtomicUsize, // atomic to avoid mut on `fetch_next()`, but it's a steel non thread-safe struct
}

impl<T> CycleArray<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner, idx: AtomicUsize::new(0) }
    }

    pub fn fetch_next(&self) -> Option<&T> {
        if self.inner.len() == 0 { return None }
        let to = (self.idx.load(Relaxed) + 1) % self.inner.len();
        let prev = self.idx.swap(to, Relaxed);
        Some(&self.inner[prev])
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    pub fn remove(&mut self, idx: usize) -> Option<T> {
        if self.inner.len() == 0 { return None }
        let removed = self.inner.swap_remove(idx);
        if self.idx.load(Relaxed) >= self.inner.len() {
            self.idx.store(0, Relaxed);
        }
        Some(removed)
    }

    pub fn head(&self) -> usize {
        self.idx.load(Relaxed)
    }
}

pub struct JoinHandlePool<T: Send + 'static> {
    handles: Arc<RwLock<CycleArray<JoinHandle<T>>>>,
    complete: Mutex<Receiver<usize>>, 
}

impl<T> JoinHandlePool<T>
where 
    T: Send + 'static
{
    pub fn with_capacity(capacity: usize) -> Self {
        let (res_tx, res_rx) = mpsc::channel(capacity);
        let handles = Arc::new(RwLock::new(CycleArray::<JoinHandle<T>>::new(Vec::with_capacity(capacity))));

        let pool = Self { handles, complete: Mutex::new(res_rx) };
        pool.run_worker(res_tx);
        pool
    }

    fn run_worker(&self, res_tx: Sender<usize>) {
        let handles = self.handles.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                let lock = handles.read().await;
                if let Some(h) = lock.fetch_next() {
                    if !h.is_finished() {
                        continue
                    }

                    if res_tx.send(lock.head()).await.is_err() {
                        break
                    }
                }
            }
        });
    }

    pub async fn close(&self) {
        self.complete.lock().await.close();
    }

    pub async fn push_future<F>(&self, future: F) 
    where 
        F: Future<Output = T> + Send + 'static,
    {
        self.handles.write().await.push(tokio::spawn(future));
    }

    pub async fn get_complete(&self) -> Option<T> {
        let complete = self.complete.lock().await.recv().await?;
        self.handles.write().await.remove(complete)?.await.ok()
    }
} 
use {
    std::{collections::HashMap, sync::Arc}, tokio::sync::{broadcast::{Receiver, Sender, channel}, RwLock}, uuid::Uuid,
};

pub struct ConnectionInner {
    is_upload: bool,
    filename: String,
    chunks_count: i32,
    loaded: i32, // count of saved or loaded chunks
    cancel: Sender<()>,
}

impl ConnectionInner {
    pub fn new(filename: String, chunks_count: i32) -> Self {
        Self { is_upload: false, filename, chunks_count, loaded: 0, cancel: channel::<()>(1).0 }
    }

    pub fn upload_conn(mut self) -> Self {
        self.is_upload = true;
        self
    }

    pub fn cancel_receiver(&self) -> Receiver<()> {
        self.cancel.subscribe()
    } 

    pub fn cancel(&self) {
        let _ = self.cancel.send(());
    }
}

pub type FileProgress = (Uuid, bool, String, f32);

#[derive(Clone)]
pub struct Connections {
    inner: Arc<RwLock<HashMap<Uuid, ConnectionInner>>>
}

impl Connections {
    pub fn new() -> Self {
        let s = Self { inner: Arc::new(RwLock::new(HashMap::new())) };
        s.start_cleaner();
        s
    }

    fn start_cleaner(&self) {
        let conns = self.inner.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                let mut end_conns = Vec::<Uuid>::with_capacity(conns.read().await.len());
                {
                    let r_lock = conns.read().await;
                        for (id, conn) in r_lock.iter() {
                        if conn.loaded >= conn.chunks_count {
                            end_conns.push(*id);
                        }
                    }
                }
                let mut w_lock = conns.write().await;
                for id in end_conns {
                    w_lock.remove(&id);
                } 
            }            
        });
    } 

    pub async fn progress_list(&self) -> Vec<FileProgress> {
        let lock = self.inner.read().await;
        let mut list = Vec::<FileProgress>::with_capacity(lock.len());
        for (id, v) in lock.iter() {
            list.push((id.clone(), v.is_upload, v.filename.clone(), v.loaded as f32 / v.chunks_count as f32));
        }
        list
    }

    pub async fn add(&mut self, key: Uuid, val: ConnectionInner) {
        self.inner.write().await.insert(key, val);
    }

    pub async fn cancel(&mut self, key: Uuid) {
        let mut lock = self.inner.write().await;
        if let Some(conn) = lock.get(&key) {
            conn.cancel()
        };
        lock.remove(&key);
    }

    pub async fn increase_progress(&mut self, id: Uuid) -> bool {
        if let Some(conn) = self.inner.write().await.get_mut(&id) {
            conn.loaded += 1;
            return true;
        }
        false
    }
}
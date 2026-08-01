use {
    std::{collections::HashMap, sync::Arc}, 
    tokio::sync::{RwLock, broadcast::{Receiver, Sender, channel}}, 
    uuid::Uuid,
};

pub struct ConnectionInner {
    is_upload: bool,
    filename: String,

    chunks_count: i32,
    loaded: i32, // count of saved or loaded chunks
    previous: i32,

    cancel: Sender<()>,
}

impl ConnectionInner {
    pub fn new(filename: String, chunks_count: i32) -> Self {
        Self { is_upload: false, filename, chunks_count, loaded: 0, previous: 0, cancel: channel::<()>(1).0 }
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

pub struct ConnectionInfo {
    pub id: Uuid,
    pub is_upload: bool,
    pub filename: String,
    pub load_progress: f32,
    pub previous_progress: f32,
}

impl ConnectionInfo {
    pub fn new(id: Uuid, conn_inner: &ConnectionInner) -> Self {
        let chunks_count = conn_inner.chunks_count as f32;
        Self { id, is_upload: conn_inner.is_upload, filename: conn_inner.filename.clone(), load_progress: conn_inner.loaded as f32 / chunks_count, previous_progress: conn_inner.previous as f32 / chunks_count }
    }
}

pub type CompletedLoad = (String, bool);

#[derive(Clone)]
pub struct Connections {
    inner: Arc<RwLock<HashMap<Uuid, ConnectionInner>>>,
    completed: Arc<RwLock<Vec<CompletedLoad>>>
}

impl Connections {
    pub fn new() -> Self {
        let s = Self { 
            inner: Arc::new(RwLock::new(HashMap::new())),
            completed: Arc::new(RwLock::new(Vec::new()))
        };
        s.start_cleaner();
        s
    }

    fn start_cleaner(&self) {
        let conns = self.inner.clone();
        let completed = self.completed.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
            loop {
                interval.tick().await;
                let to_remove: Vec<Uuid> = {
                    let lock = conns.read().await;
                    lock.iter()
                        .filter(|(_, v)| v.loaded >= v.chunks_count)
                        .map(|(k, _)| *k)
                        .collect()
                };

                if to_remove.is_empty() {
                    continue;
                }

                let mut lock = conns.write().await;
                let completed_items: Vec<CompletedLoad> = to_remove.into_iter()
                    .filter_map(|k| lock.remove(&k))
                    .map(|v| (v.filename, v.is_upload))
                    .collect();

                if !completed_items.is_empty() {
                    completed.write().await.extend(completed_items);
                }
            }            
        });
    } 

    pub async fn completed(&self) -> Vec<CompletedLoad> {
        let mut lock = self.completed.write().await;
        lock.drain(..).collect()
    } 

    pub async fn progress_list(&self) -> Vec<ConnectionInfo> {
        let mut lock = self.inner.write().await;
        let mut list = Vec::<ConnectionInfo>::with_capacity(lock.len());
        for (id, v) in lock.iter_mut() {
            list.push(ConnectionInfo::new(id.clone(), v));
            v.previous = v.loaded;
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
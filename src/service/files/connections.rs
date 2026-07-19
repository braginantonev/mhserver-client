use {
    std::{collections::HashMap, sync::{Arc, Mutex}},
    uuid::Uuid,
};

pub struct ConnectionInner {
    filename: String,
    chunk_size: i64,
    chunks_count: i32,
    loaded: i32, // count of saved or loaded chunks  
}

impl ConnectionInner {
    pub fn new(filename: String, chunk_size: i64, chunks_count: i32) -> Self {
        Self { filename, chunk_size, chunks_count, loaded: 0 }
    }
}

pub type FileProgress = (Uuid, String, f32);

#[derive(Clone)]
pub struct Connections {
    inner: Arc<Mutex<HashMap<Uuid, ConnectionInner>>>
}

impl Connections {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn progress_list(&self) -> Vec<FileProgress> {
        let lock = self.inner.lock().unwrap();
        let mut list = Vec::<FileProgress>::with_capacity(lock.len());
        for (id, v) in lock.iter() {
            list.push((id.clone(), v.filename.clone(), v.loaded as f32 / v.chunks_count as f32));
        }
        list
    }

    pub fn add(&mut self, key: Uuid, val: ConnectionInner) {
        self.inner.lock().unwrap().insert(key, val);
    }

    pub fn increase_progress(&mut self, id: Uuid) -> bool {
        let mut lock = self.inner.lock().unwrap();
        if let Some(conn) = lock.get_mut(&id) {
            if conn.loaded + 1 >= conn.chunks_count {
                lock.remove(&id);
            } else {
                conn.loaded += 1;
            }
            return true;
        }
        false
    }
}
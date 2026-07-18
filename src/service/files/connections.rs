
use {
    std::{collections::HashMap, sync::{Arc, Mutex}},
    uuid::Uuid,
};

pub struct ConnectionInner {
    chunk_size: i64,
    chunks_count: i32,
    loaded: i32, // count of saved or loaded chunks  
}

impl ConnectionInner {
    pub fn new(chunk_size: i64, chunks_count: i32) -> Self {
        Self { chunk_size, chunks_count, loaded: 0 }
    }
}

#[derive(Clone)]
pub struct Connections {
    inner: Arc<Mutex<HashMap<Uuid, ConnectionInner>>>
}

impl Connections {
    pub fn new() -> Self {
        Self { inner: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn add(&mut self, key: Uuid, val: ConnectionInner) {
        self.inner.lock().unwrap().insert(key, val);
    }

    pub fn progress(&self, id: Uuid) -> f32 {
        let lock = self.inner.lock().unwrap();
        lock[&id].loaded as f32 / lock[&id].chunks_count as f32
    }

    pub fn increase_progress(&mut self, id: Uuid) -> bool {
        if let Some(conn) = self.inner.lock().unwrap().get_mut(&id) {
            conn.loaded += 1;
            return true;
        }
        false
    }
}
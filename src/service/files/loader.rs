use {
    std::{collections::HashMap, sync::Arc},
    tokio::sync::{RwLock, Mutex},
};

#[derive(Clone)]
pub struct Load {
    is_upload: bool,
    file: String,
    to_load: Option<usize>,
    loaded: usize, // count of saved or loaded chunks
}

impl Load {
    pub fn new(is_upload: bool, file: String, to_load: Option<usize>) -> Self {
        Self {
            is_upload,
            file,
            to_load,
            loaded: 0,
        }
    }

    pub fn is_upload(&self) -> bool {
        self.is_upload
    }

    pub fn file(&self) -> String {
        self.file.clone()
    }

    /// Return percent between 0.0 and 1.0 if `to_load` size is known.
    /// If `to_load` size not set - return loaded size
    pub fn completed(&self) -> Result<f32, usize> {
        match self.to_load {
            Some(v) => Ok(v as f32 / self.loaded as f32),
            None => Err(self.loaded)
        }
    }
}

pub type LoadID = u64;

pub struct Loader {
    counter: Mutex<u64>,
    loads: Arc<RwLock<HashMap<LoadID, Load>>>,
    completed: Arc<RwLock<Vec<Load>>>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            loads: Arc::default(),
            completed: Arc::default(),
            counter: Mutex::new(0),
        }    
    }

    pub async fn drain_completed(&self) -> Vec<Load> {
        let mut lock = self.completed.write().await;
        lock.drain(..).collect()
    }

    pub async fn get_uncompleted(&self) -> Vec<(LoadID, Load)> {
        let lock = self.loads.read().await;
        lock.iter().map(|v| (*v.0, v.1.clone())).collect()
    }

    pub async fn add(&self, val: Load) -> u64 {
        let mut counter = self.counter.lock().await;
        let id = *counter;

        self.loads.write().await.insert(id, val);
        *counter += 1;

        id
    }

    /// Remove load from indexing.
    /// `Cancel()` don't push canceled loads to completed (in any case canceled task is not completed).
    pub async fn cancel(&self, id: u64) {
        self.loads.write().await.remove(&id);
    }

    /// Push `Load` to completed. Do nothing if `Load` not found.
    pub async fn set_complete(&self, id: u64) {
        if let Some(to_remove) = self.loads.write().await.remove(&id) {
            self.completed.write().await.push(to_remove);
        }
    }

    /// Return true if `loaded` from `Load` was increased.
    pub async fn increase_loaded(&self, id: u64, v: usize) -> bool {
        if let Some(conn) = self.loads.write().await.get_mut(&id) {
            conn.loaded += v;
            return true;
        }
        false
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}
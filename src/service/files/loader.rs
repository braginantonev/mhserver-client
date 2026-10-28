use {
    super::Size,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::{RwLock, Mutex},
};

#[derive(Clone, Copy, PartialEq)]
pub enum LoadType {
    Upload,
    Download
}

#[derive(Clone, Copy, PartialEq)]
pub enum LoadStatus {
    Active,
    Completed,
    Dropped
}

#[derive(Clone)]
pub struct Load {
    status: LoadStatus,
    typ: LoadType,
    file: String,
    to_load: Option<Size>,
    loaded: Size, // count of saved or loaded chunks
}

impl Load {
    pub fn new(typ: LoadType, file: String, to_load: Option<Size>) -> Self {
        Self {
            status: LoadStatus::Active,
            typ,
            file,
            to_load,
            loaded: Size(0),
        }
    }

    pub fn status(&self) -> LoadStatus {
        self.status
    }

    pub fn typ(&self) -> LoadType {
        self.typ
    }

    pub fn file(&self) -> String {
        self.file.clone()
    }

    pub fn loaded(&self) -> Size {
        self.loaded.clone()
    }

    /// Return loaded percent between 0.0 and 1.0 if `to_load` size is known
    pub fn completed_percent(&self) -> Option<f32> {
        match &self.to_load {
            Some(v) => Some(v.0 as f32 / self.loaded.0 as f32),
            None => None
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

    async fn transfer_load(&self, id: u64, new_status: LoadStatus) {
        if let Some(mut to_remove) = self.loads.write().await.remove(&id) {
            to_remove.status = new_status;
            self.completed.write().await.push(to_remove);
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

    /// Push `Load` to completed with status `LoadStatus::Dropped`. Do nothing if `Load` not found.
    /// This is a wrap func for `Loader::drop_load()`
    pub async fn cancel_load(&self, id: u64) {
        self.drop_load(id).await;
    }

    /// Push `Load` to completed. Do nothing if `Load` not found.
    pub async fn complete_load(&self, id: u64) {
        self.transfer_load(id, LoadStatus::Completed).await;
    }

    /// Push `Load` to completed with status `LoadStatus::Dropped`. Do nothing if `Load` not found.
    pub async fn drop_load(&self, id: u64) {
        self.transfer_load(id, LoadStatus::Dropped).await;
    }

    /// Return true if `loaded` from `Load` was increased.
    pub async fn increase_loaded(&self, id: u64, v: usize) -> bool {
        if let Some(conn) = self.loads.write().await.get_mut(&id) {
            conn.loaded.0 += v;
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
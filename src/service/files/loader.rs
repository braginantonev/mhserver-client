use {
    std::{collections::HashMap, sync::{Arc, atomic::{AtomicUsize, Ordering::Relaxed}}}, tokio::sync::{OnceCell, Mutex, RwLock},
};

#[derive(Clone, Copy, PartialEq)]
pub enum LoadKind {
    Upload,
    Download
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadState {
    Active,
    Completed,
    Dropped
}

pub type Parts = usize;
pub type MutableParts = AtomicUsize;

#[derive(Clone)]
pub struct LoadInfo {
    kind: LoadKind,
    target: String,
    parts: Option<Parts>
}

impl LoadInfo {
    pub fn new(kind: LoadKind, target: String, parts: Option<Parts>) -> Self {
        Self { 
            kind, 
            target, 
            parts: parts
        }
    }

    pub fn kind(&self) -> LoadKind {
        self.kind
    }

    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn parts(&self) -> Option<Parts> {
        self.parts
    }
}

pub struct LoadStatus {
    state: OnceCell<LoadState>,
    completed: MutableParts // count of saved or loaded parts
}

impl LoadStatus {
    fn set_state(&self, value: LoadState) {
        if let Err(err) = self.state.set(value) {
            eprintln!("failed set state ({:?}) for load: {}", value, err);
        };
    }

    pub fn state(&self) -> LoadState {
        self.state.get().map(|x| *x).unwrap_or(LoadState::Active)
    }

    pub fn add_progress(&self, progress: usize) {
        self.completed.fetch_add(progress, Relaxed);
    }

    pub fn completed(&self) -> usize {
        self.completed.load(Relaxed)
    }
}

impl Default for LoadStatus {
    fn default() -> Self {
        Self { state: OnceCell::new(), completed: AtomicUsize::new(0) }
    }
}

pub struct Load {
    info: LoadInfo,
    status: LoadStatus,
}

impl Load {
    pub fn new(info: LoadInfo) -> Self {
        Self {
            info,
            status: LoadStatus::default()
        }
    }

    pub fn info(&self) -> &LoadInfo {
        &self.info
    }

    pub fn status(&self) -> &LoadStatus {
        &self.status
    }
}

pub type LoadID = usize;
pub type LoadToken = Arc<Load>;

pub struct Loader {
    counter: AtomicUsize,
    loads: RwLock<HashMap<LoadID, Arc<Load>>>,
    ended: Mutex<Vec<Arc<Load>>>
}

impl Loader {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
            loads: RwLock::new(HashMap::new()),
            ended: Mutex::new(Vec::new()),
        }    
    }

    async fn transfer_drop_load(&self, id: LoadID, drop_state: LoadState) {
        if let Some(to_remove) = self.loads.write().await.remove(&id) {
            to_remove.status().set_state(drop_state);
            self.ended.lock().await.push(to_remove);
        }
    }

    fn free_id(&self) -> LoadID {
        self.counter.fetch_add(1, Relaxed)
    }

    pub async fn add(&self, info: LoadInfo) -> (LoadID, LoadToken) {
        let load = Arc::new(Load::new(info));
        let id = self.free_id();
        self.loads.write().await.insert(id, load.clone());
        (id, load)
    }

    pub async fn drain_ended(&self) -> Vec<Arc<Load>> {
        let mut lock = self.ended.lock().await;
        lock.drain(..).collect()
    }

    pub async fn get_active(&self) -> Vec<(LoadID, LoadToken)> {
        let lock = self.loads.read().await;
        lock.iter().map(|v| (*v.0, v.1.clone())).collect()
    }

    /// Push `Load` to completed with status `LoadStatus::Dropped`. Do nothing if `Load` not found.
    /// This is a wrap func for `Loader::drop_load()`
    pub async fn cancel_load(&self, id: LoadID) {
        self.drop_load(id).await;
    }

    /// Push `Load` to completed. Do nothing if `Load` not found.
    pub async fn complete_load(&self, id: LoadID) {
        self.transfer_drop_load(id, LoadState::Completed).await;
    }

    /// Push `Load` to completed with status `LoadStatus::Dropped`. Do nothing if `Load` not found.
    pub async fn drop_load(&self, id: LoadID) {
        self.transfer_drop_load(id, LoadState::Dropped).await;
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}
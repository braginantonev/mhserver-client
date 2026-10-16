use {
    std::sync::Arc, 
    tokio::{time::Duration, sync::mpsc::{Receiver, Sender, channel}},
};

//todo: use on ping services, when this will be possible in server API
/*#[allow(dead_code)] 
pub fn get_delay(rl: &api::apis::RateLimit) -> Duration {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    Duration::from_millis((rl.reset() - now) * 1000 / rl.limit() as u64)
}
*/

/// The request queue provided the wait() to distribute the sending of requests to the ratelimit time window.
/// This is simple lock with one timer.
pub struct Limiter {
    rx: Arc<tokio::sync::Mutex<Receiver<()>>>,
}

impl Limiter {
    pub fn new(pass_interval: Duration) -> Self {
        let (tx, rx) = channel::<()>(1);
        let queue = Self { rx: Arc::new(tokio::sync::Mutex::new(rx)) };
        queue.start(tx, pass_interval);
        queue
    }

    fn start(&self, tx: Sender<()>, interval: Duration) {
        tokio::spawn(async move {
            loop {
                let _ = tx.send(()).await;
                tokio::time::sleep(interval).await;
            }
        });
    }

    pub async fn wait(&self) {
        let rx = self.rx.clone();
        rx.lock().await.recv().await;
    }
}

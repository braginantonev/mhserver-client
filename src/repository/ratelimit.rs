use {
    reqwest_middleware::Middleware, 
    std::{sync::Arc, time::SystemTime}, 
    tokio::{time::Duration, sync::mpsc::{Receiver, Sender, channel}},
};

//todo: use on ping services, when this will be possible in server API
#[allow(dead_code)] 
pub fn get_delay(rl: &api::apis::RateLimit) -> Duration {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    Duration::from_millis((rl.reset() - now) * 1000 / rl.limit() as u64)
}

/// The request queue provided the wait() to distribute the sending of requests to the ratelimit time window.
/// This is simple lock with one timer.
pub struct RequestQueue {
    rx: Arc<tokio::sync::Mutex<Receiver<()>>>,
}

impl RequestQueue {
    pub fn new(pass_interval: Duration) -> Self {
        let (tx, rx) = channel::<()>(1);
        let queue = Self { rx: Arc::new(tokio::sync::Mutex::new(rx)) };
        queue.start_passing(tx, pass_interval);
        queue
    }

    fn start_passing(&self, tx: Sender<()>, interval: Duration) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                let _ = tx.send(()).await;
                interval.tick().await;
            }
        });
    }

    pub async fn wait(&self) {
        let rx = self.rx.clone();
        rx.lock().await.recv().await;
    }
}

pub struct RateLimitMiddleware {
    queue: RequestQueue
}

impl RateLimitMiddleware {
    pub fn new(queue: RequestQueue) -> Self {
        Self { queue }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimitMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        extensions: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.queue.wait().await;
        let resp = next.run(req, extensions).await?;
        Ok(resp)
    }
}
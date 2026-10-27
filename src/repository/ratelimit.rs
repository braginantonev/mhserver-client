use {
    std::sync::Arc,
    tokio::{
        sync::{
            Mutex,
            mpsc::{Receiver, Sender, channel},
        },
        time::Duration,
    },
};

#[derive(Clone)]
pub struct Rate {
    limit: usize,
    window: Duration,
}

impl Rate {
    pub fn new(limit: usize, time_window: Duration) -> Self {
        Self { limit, window: time_window }
    }
}

type Token = ();

/// The request queue provided the wait() to distribute the sending of requests to the ratelimit time window.
/// This is thread-safe struct.
#[derive(Clone)]
pub struct Limiter {
    receiver: Arc<Mutex<Receiver<Token>>>
}

impl Limiter {
    pub fn new(rate: Rate) -> Self {
        let (rx, tx) = channel(rate.limit); 
        let limiter = Self { receiver: Arc::new(Mutex::new(tx)) };
        limiter.start(rx, rate.window / rate.limit as u32);
        limiter
    }

    fn start(&self, rx: Sender<Token>, interval: Duration) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                let _ = rx.send(()).await;
            }
        });
    }

    pub async fn wait(&self) {
        self.receiver.lock().await.recv().await;
    }
}

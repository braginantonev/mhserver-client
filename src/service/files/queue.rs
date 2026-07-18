use {
    std::{sync::Arc, time::Duration}, 
    tokio::sync::mpsc::{Sender, Receiver, channel},
};

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
            loop {
                tx.send(()).await.unwrap();
                tokio::time::sleep(interval).await;
            }
        });
    }

    pub async fn wait(&self) {
        let rx = self.rx.clone();
        rx.lock().await.recv().await;
    }
}
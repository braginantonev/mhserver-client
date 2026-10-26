use {
    tokio::{
        sync::mpsc::{Receiver, Sender, channel},
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
/// This is simple non thread-safe struct.
pub struct Limiter {
    rate: Rate,
    receiver: Receiver<Token>
}

impl Limiter {
    pub fn new(rate: Rate) -> Self {
        let (rx, tx) = channel(rate.limit); 
        let limiter = Self {
            rate: rate,
            receiver: tx,
        };

        limiter.start(rx);
        limiter
    }

    fn start(&self, rx: Sender<Token>) {
        let mut ticker = tokio::time::interval(self.rate.window / self.rate.limit as u32);
        tokio::spawn(async move {
            loop {
                ticker.tick().await;
                let _ = rx.send(()).await;
            }
        });
    }

    pub fn wait(&mut self) {
        self.receiver.blocking_recv();
    }
}

impl Clone for Limiter {
    fn clone(&self) -> Self {
        Self::new(self.rate.clone())
    }
}

use {
    std::time::SystemTime,
    tokio::time::Duration
};

pub fn get_delay(rl: &api::apis::RateLimit) -> Duration {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    Duration::from_millis((rl.reset() - now) * 1000 / rl.limit() as u64)
}
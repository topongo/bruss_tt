use tokio::sync::Mutex;

pub(crate) type StatsType = Mutex<Stats>;

#[derive(Debug)]
pub(crate) struct Stats {
    pub requests: usize,
    // pub bytes_sent: u64,
    pub bytes_received: usize,
    // pub errors: u64,
    pub instant: std::time::Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self { requests: 0, bytes_received: 0, instant: std::time::Instant::now() }
    }
}


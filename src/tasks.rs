use tokio::sync::watch;

#[derive(Clone)]
pub struct RunChecker {
    keep_running: watch::Receiver<bool>,
}

impl RunChecker {
    pub fn new(keep_running: watch::Receiver<bool>) -> RunChecker {
        RunChecker { keep_running }
    }

    pub fn keep_running(&self) -> bool {
        *self.keep_running.borrow()
    }

    pub async fn shutdown_recv(&mut self) {
        let _ = self.keep_running.changed().await;
    }
}

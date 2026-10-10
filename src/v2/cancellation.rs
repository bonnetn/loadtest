use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use event_listener::Event;

#[derive(Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
    event: Arc<Event>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            event: Arc::new(Event::new()),
        }
    }

    pub fn cancel(&self) {
        if !self.cancelled.swap(true, Ordering::Release) {
            self.event.notify(usize::MAX);
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    pub async fn cancelled(&self) {
        if self.is_cancelled() {
            return;
        }

        let listener = self.event.listen();
        if !self.is_cancelled() {
            listener.await;
        }
    }
}

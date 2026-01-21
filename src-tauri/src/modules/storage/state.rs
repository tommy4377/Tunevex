use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Default)]
pub struct CompactorState {
    pub cancel_token: Arc<AtomicBool>,
}

impl CompactorState {
    pub fn cancel(&self) {
        self.cancel_token.store(true, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.cancel_token.store(false, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.load(Ordering::SeqCst)
    }
}

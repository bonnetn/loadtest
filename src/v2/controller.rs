use super::worker::WorkerHandle;

#[derive(Debug, Default)]
pub struct Controller {
    handles: Vec<WorkerHandle>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_handles(handles: Vec<WorkerHandle>) -> Self {
        Self { handles }
    }

    pub fn handles(&self) -> &[WorkerHandle] {
        &self.handles
    }

    pub fn add_handle(&mut self, handle: WorkerHandle) {
        self.handles.push(handle);
    }
}

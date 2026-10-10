#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Worker {
    id: usize,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(self) -> usize {
        self.id
    }
}

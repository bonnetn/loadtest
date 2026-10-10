use miette::Result;

pub struct Tester;

impl Tester {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}

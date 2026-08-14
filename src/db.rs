#[derive(Clone)]
pub struct SpaceTimeClient {
    pub base_url: String,
}

impl SpaceTimeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub async fn ping(&self) -> Result<(), ()> {
        // TODO: implement SpaceTime DB integration
        Ok(())
    }
}

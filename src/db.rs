#[derive(Clone)]
pub struct SpaceTimeClient {
    #[allow(dead_code)]
    pub base_url: String,
}

impl SpaceTimeClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    #[allow(dead_code)]
    pub async fn ping(&self) -> Result<(), ()> {
        // TODO: implement SpaceTime DB integration
        Ok(())
    }
}

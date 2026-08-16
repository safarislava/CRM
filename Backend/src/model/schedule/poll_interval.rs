use crate::model::contract::box_error::BoxError;
use crate::model::schedule::contract::event::Event;
use std::time::Duration;

pub struct PollInterval {
    duration: Duration,
}

impl PollInterval {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

#[async_trait::async_trait]
impl Event for PollInterval {
    async fn fired(&self) -> Result<(), BoxError> {
        actix_web::rt::time::sleep(self.duration).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn fires_after_duration() {
        let interval = PollInterval::new(Duration::from_millis(5));
        let start = std::time::Instant::now();
        interval.fired().await.unwrap();
        assert!(start.elapsed() >= Duration::from_millis(5));
    }
}

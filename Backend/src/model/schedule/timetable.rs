use crate::model::contract::box_error::BoxError;
use crate::model::schedule::contract::scheduled::Scheduled;
use crate::model::schedule::schedule::Schedule;
use futures_util::future::join_all;

pub struct Timetable {
    schedules: Vec<Schedule>,
}

impl Timetable {
    pub fn new(schedules: Vec<Schedule>) -> Self {
        Self { schedules }
    }
}

#[async_trait::async_trait]
impl Scheduled for Timetable {
    async fn run(&self) -> Result<(), BoxError> {
        let futures = self.schedules.iter().map(|s| async move {
            if let Err(error) = s.run().await {
                tracing::error!(error = %error, "Schedule runner terminated unexpectedly");
            }
        });
        join_all(futures).await;
        Ok(())
    }
}

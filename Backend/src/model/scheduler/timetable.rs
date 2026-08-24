use crate::model::contract::box_error::BoxError;
use crate::model::scheduler::contract::schedule::Schedule;
use crate::model::scheduler::scheduled_task::ScheduledTask;
use futures_util::future::join_all;

pub struct Timetable {
    schedules: Vec<ScheduledTask>,
}

impl Timetable {
    pub fn new(schedules: Vec<ScheduledTask>) -> Self {
        Self { schedules }
    }
}

#[async_trait::async_trait]
impl Schedule for Timetable {
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

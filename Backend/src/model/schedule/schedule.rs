use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::schedule::contract::event::Event;
use crate::model::schedule::contract::scheduled::Scheduled;
use std::sync::Arc;

pub struct Schedule {
    event: Arc<dyn Event>,
    task: Arc<dyn Task<Output = ()> + Send + Sync>,
}

impl Schedule {
    pub fn new(event: Arc<dyn Event>, task: Arc<dyn Task<Output = ()> + Send + Sync>) -> Self {
        Self { event, task }
    }
}

#[async_trait::async_trait]
impl Scheduled for Schedule {
    async fn run(&self) -> Result<(), BoxError> {
        loop {
            if let Err(error) = self.event.fired().await {
                tracing::error!(error = %error, "Schedule event trigger error");
                continue;
            }
            if let Err(error) = self.task.perform().await {
                tracing::error!(error = %error, "Schedule task execution error");
            }
        }
    }
}

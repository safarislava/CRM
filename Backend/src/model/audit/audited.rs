use crate::common::BoxError;
use crate::model::contract::task::Task;
use std::fmt::Display;

pub struct AuditedTask<T, S, A, O> {
    subject: S,
    action: A,
    object: O,
    task: T,
}

impl<T, S, A, O> AuditedTask<T, S, A, O> {
    pub fn new(subject: S, action: A, object: O, task: T) -> Self {
        Self {
            subject,
            action,
            object,
            task,
        }
    }
}

#[async_trait::async_trait]
impl<T, S, A, O, Out> Task for AuditedTask<T, S, A, O>
where
    T: Task<Output = Out> + Send + Sync,
    S: Display + Send + Sync,
    A: Display + Send + Sync,
    O: Display + Send + Sync,
    Out: Send + Sync,
{
    type Output = Out;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        let result = self.task.perform().await;
        match &result {
            Ok(_) => {
                tracing::info!(
                    subject = %self.subject,
                    action = %self.action,
                    object = %self.object,
                    "succeed"
                );
            }
            Err(err) => {
                tracing::error!(
                    subject = %self.subject,
                    action = %self.action,
                    object = %self.object,
                    error = %err,
                    "failed"
                );
            }
        }
        result
    }
}

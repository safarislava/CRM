use crate::model::contract::box_error::BoxError;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::audit::action::AuditAction;

    struct ValueTask<V>(V);

    #[async_trait::async_trait]
    impl<V: Clone + Send + Sync> Task for ValueTask<V> {
        type Output = V;
        async fn perform(&self) -> Result<Self::Output, BoxError> {
            Ok(self.0.clone())
        }
    }

    struct FailingTask;

    #[async_trait::async_trait]
    impl Task for FailingTask {
        type Output = ();
        async fn perform(&self) -> Result<Self::Output, BoxError> {
            Err("task error".into())
        }
    }

    #[actix_web::test]
    async fn performs_task_and_returns_output_on_success() {
        let task = AuditedTask::new("user_1", AuditAction::UserCreate, "obj_1", ValueTask(100));
        let res = task.perform().await;
        assert_eq!(res.unwrap(), 100);
    }

    #[actix_web::test]
    async fn propagates_error_on_failure() {
        let task = AuditedTask::new("user_1", AuditAction::ProjectDelete, "obj_1", FailingTask);
        let res = task.perform().await;
        assert!(res.is_err());
    }
}

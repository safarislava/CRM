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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    struct TestEvent(Duration);

    #[async_trait::async_trait]
    impl Event for TestEvent {
        async fn fired(&self) -> Result<(), BoxError> {
            actix_web::rt::time::sleep(self.0).await;
            Ok(())
        }
    }

    struct CountTask(Arc<AtomicUsize>);

    #[async_trait::async_trait]
    impl Task for CountTask {
        type Output = ();
        async fn perform(&self) -> Result<(), BoxError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[actix_web::test]
    async fn executes_task_when_event_fires() {
        let counter = Arc::new(AtomicUsize::new(0));
        let event = Arc::new(TestEvent(Duration::from_millis(10)));
        let task = Arc::new(CountTask(counter.clone()));
        let schedule = Arc::new(Schedule::new(event, task));

        let runner = schedule.clone();
        let handle = actix_web::rt::spawn(async move {
            let _ = runner.run().await;
        });

        actix_web::rt::time::sleep(Duration::from_millis(35)).await;
        handle.abort();

        assert!(counter.load(Ordering::SeqCst) >= 2);
    }
}

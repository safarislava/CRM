use crate::model::contract::box_error::BoxError;
use crate::model::scheduler::contract::event::Event;
use chrono::Local;
use cron::Schedule as CronSchedule;
use std::str::FromStr;

pub struct CronEvent {
    schedule: CronSchedule,
}

impl CronEvent {
    pub fn new(expression: &str) -> Result<Self, BoxError> {
        let schedule = CronSchedule::from_str(expression)?;
        Ok(Self { schedule })
    }
}

#[async_trait::async_trait]
impl Event for CronEvent {
    async fn fired(&self) -> Result<(), BoxError> {
        let now = Local::now();
        if let Some(next) = self.schedule.upcoming(Local).next()
            && next > now
        {
            let duration = (next - now).to_std()?;
            actix_web::rt::time::sleep(duration).await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_cron_expression() {
        let event = CronEvent::new("0 0 12 * * * *");
        assert!(event.is_ok());
    }

    #[test]
    fn rejects_invalid_cron_expression() {
        let event = CronEvent::new("invalid cron expression");
        assert!(event.is_err());
    }
}

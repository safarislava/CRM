mod cors;
mod db;
mod endpoint;
mod jwt;
mod logger;
mod mail;
mod middleware;
mod model;
mod routes;
mod state;
mod storage;

use crate::logger::{AppLogs, RollingLogs};
use crate::mail::Mailer;
use crate::model::notification::deadline_digest_notification::DeadlineDigestNotification;
use crate::model::notification::notification_dispatch::NotificationDispatch;
use crate::model::schedule::contract::scheduled::Scheduled;
use crate::model::schedule::poll_interval::PollInterval;
use crate::model::schedule::schedule::Schedule;
use crate::model::schedule::time_of_day::TimeOfDay;
use crate::model::schedule::timetable::Timetable;
use crate::state::AppState;
use crate::storage::Storage;
use actix_web::{web, App, HttpServer};
use chrono::NaiveTime;
use std::sync::Arc;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let _log_guard = RollingLogs::default_crm().attach();
    tracing::info!("Starting DailyCRM server...");
    let pool = Arc::new(db::pool().await);
    let storage = Arc::new(Storage::from_env().await);
    let mailer = Arc::new(Mailer::from_env());
    let state = web::Data::new(AppState {
        pool: pool.clone(),
        storage: storage.clone(),
        mailer: mailer.clone(),
    });
    let deadline_schedule = Schedule::new(
        Arc::new(TimeOfDay::new(NaiveTime::from_hms_opt(12, 0, 0).unwrap())),
        Arc::new(DeadlineDigestNotification::new(
            pool.clone(),
            mailer.clone(),
        )),
    );
    let dispatch_schedule = Schedule::new(
        Arc::new(PollInterval::new(Duration::from_mins(1))),
        Arc::new(NotificationDispatch::new(pool, mailer)),
    );
    let timetable = Timetable::new(vec![deadline_schedule, dispatch_schedule]);
    actix_web::rt::spawn(async move {
        if let Err(error) = timetable.run().await {
            tracing::error!("schedule stopped: {error}");
        }
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(cors::rules())
            .wrap(cors::security_headers())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// TODO Backend tests

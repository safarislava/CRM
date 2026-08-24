use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::notification::enqueue::NotificationEnqueue;
use crate::model::project::contract::file::File;
use crate::model::project::stage::act::upload::ActUpload;
use crate::model::project::stage::id::StageId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage_id: StageId,
    file: Arc<dyn File>,
}

impl NotifiedActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage_id: StageId,
        file: Arc<dyn File>,
    ) -> Self {
        Self {
            pool,
            storage,
            stage_id,
            file,
        }
    }
}

#[async_trait::async_trait]
impl Task for NotifiedActUpload {
    type Output = ();

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        ActUpload::new(
            self.pool.clone(),
            self.storage.clone(),
            self.stage_id,
            self.file.clone(),
        )
        .perform()
        .await?;
        NotificationEnqueue::new(self.pool.clone(), self.stage_id, "act_uploaded")
            .perform()
            .await
    }
}

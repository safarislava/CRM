use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::notification::notification_enqueue::NotificationEnqueue;
use crate::model::project::file_content::FileContent;
use crate::model::project::stage::act::act_upload::ActUpload;
use crate::model::project::stage::stage_id::StageId;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct NotifiedActUpload {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    stage_id: StageId,
    file: FileContent,
}

impl NotifiedActUpload {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        stage_id: StageId,
        file: FileContent,
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
            self.stage_id.clone(),
            self.file.clone(),
        )
        .perform()
        .await?;
        NotificationEnqueue::new(self.pool.clone(), self.stage_id.clone(), "act_uploaded")
            .perform()
            .await
    }
}

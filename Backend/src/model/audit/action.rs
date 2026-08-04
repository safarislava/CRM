use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditAction {
    UserCreate,
    InviteCreate,
    ProjectCreate,
    ProjectRename { new_title: String },
    ProjectDelete,
    StageCreate,
    StageRename { new_title: String },
    StageDelete,
    StageReorder { to: i32 },
    DeadlineUpdate { new_deadline: Option<String> },
    AdvanceCostUpdate { new_cost: Option<i32> },
    FinalCostUpdate { new_cost: Option<i32> },
    GipConfirm { confirmed: bool },
    ActUpload { filename: String },
    ActDelete,
    AttachmentUpload { filename: String },
    AttachmentDelete { filename: String },
    CommentCreate { text: String },
    CommentDelete,
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserCreate => write!(f, "user_create"),
            Self::InviteCreate => write!(f, "invite_create"),
            Self::ProjectCreate => write!(f, "project_create"),
            Self::ProjectRename { new_title } => {
                write!(f, "project_rename(title: '{new_title}')")
            }
            Self::ProjectDelete => write!(f, "project_delete"),
            Self::StageCreate => write!(f, "stage_create"),
            Self::StageRename { new_title } => {
                write!(f, "stage_rename(title: '{new_title}')")
            }
            Self::StageDelete => write!(f, "stage_delete"),
            Self::StageReorder { to } => write!(f, "stage_reorder(to: {to})"),

            Self::DeadlineUpdate { new_deadline } => {
                let d = new_deadline.as_deref().unwrap_or("none");
                write!(f, "deadline_update(deadline: '{d}')")
            }
            Self::AdvanceCostUpdate { new_cost } => {
                let c = new_cost.map(|v| v.to_string()).unwrap_or_else(|| "none".to_string());
                write!(f, "advance_cost_update(cost: {c})")
            }
            Self::FinalCostUpdate { new_cost } => {
                let c = new_cost.map(|v| v.to_string()).unwrap_or_else(|| "none".to_string());
                write!(f, "final_cost_update(cost: {c})")
            }
            Self::GipConfirm { confirmed } => {
                write!(f, "gip_confirm(confirmed: {confirmed})")
            }
            Self::ActUpload { filename } => {
                write!(f, "act_upload(file: '{filename}')")
            }
            Self::ActDelete => write!(f, "act_delete"),
            Self::AttachmentUpload { filename } => {
                write!(f, "attachment_upload(file: '{filename}')")
            }
            Self::AttachmentDelete { filename } => {
                write!(f, "attachment_delete(file: '{filename}')")
            }
            Self::CommentCreate { text } => {
                write!(f, "comment_create(text: '{text}')")
            }
            Self::CommentDelete => write!(f, "comment_delete"),
        }
    }
}

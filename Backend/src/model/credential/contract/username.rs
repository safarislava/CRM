use crate::model::contract::box_error::BoxError;
use async_trait::async_trait;

#[async_trait]
pub trait Username: Send + Sync + 'static {
    async fn value(&self) -> Result<String, UsernameError>;
}

#[derive(Debug)]
pub enum UsernameError {
    TooShort,
    TooLong,
    InvalidChars,
    NotFound,
    Internal(BoxError),
}

impl std::fmt::Display for UsernameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::TooShort | Self::TooLong => f.write_str("Username must be 3–50 characters"),
            Self::InvalidChars => {
                f.write_str("Username may only contain letters, digits, spaces, _ or -")
            }
            Self::NotFound => f.write_str("Username not found"),
            Self::Internal(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for UsernameError {}

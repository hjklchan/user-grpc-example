#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),
    
    #[error("unknown error")]
    Unknown,
}

mod error;

use abi;
pub use error::UserError;

#[async_trait]
pub trait UserStore {
    async fn create() -> Result<(), UserError>;
    async fn update() -> Result<(), UserError>;
    async fn delete() -> Result<(), UserError>;
    async fn get() -> Result<(), UserError>;
    async fn list() -> Result<(), UserError>;
}
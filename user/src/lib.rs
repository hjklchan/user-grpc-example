mod error;

use abi;
pub use error::UserError;

use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, new_user: abi::User) -> Result<(), UserError>;
    async fn update(&self, user: abi::User) -> Result<(), UserError>;
    async fn delete(&self, user_id: String) -> Result<(), UserError>;
    async fn get(&self, user_id: String) -> Result<abi::User, UserError>;
    async fn list() -> Result<Vec<abi::User>, UserError>;
}
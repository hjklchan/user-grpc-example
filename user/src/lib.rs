mod error;

use abi;
pub use error::UserError;

use async_trait::async_trait;
use sqlx::{Pool, mysql::MySql};

pub struct DataStore {
    pub pool: Pool<MySql>
}

type UserId = String;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, new_user: abi::User) -> Result<(), UserError>;
    async fn update(&self, user: abi::User) -> Result<(), UserError>;
    async fn delete(&self, user_id: String) -> Result<(), UserError>;
    async fn get(&self, user_id: String) -> Result<abi::User, UserError>;
    async fn list() -> Result<Vec<abi::User>, UserError>;
}

#[async_trait]
impl UserRepo for DataStore {
    async fn create(&self, new_user: abi::User) -> Result<(), UserError> {
        todo!()
    }

    async fn update(&self, user: abi::User) -> Result<(), UserError> {
        todo!()
    }

    async fn delete(&self, user_id: UserId) -> Result<(), UserError> {
        todo!()
    }

    async fn get(&self, user_id: UserId) -> Result<abi::User, UserError> {
        todo!()
    }

    async fn list() -> Result<Vec<abi::User>, UserError> {
        todo!()
    }
}
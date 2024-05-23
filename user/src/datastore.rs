use crate::{error::UserError, user_type::UserId, UserRepo};
use async_trait::async_trait;
use sqlx::{mysql::MySql, Pool};

pub struct DataStore {
    pub pool: Pool<MySql>,
}

#[async_trait]
impl UserRepo for DataStore {
    async fn create(&self, new_user: abi::User) -> Result<(), UserError> {
        // insert query
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

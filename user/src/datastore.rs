use crate::{error::UserError, user_type::UserId, UserRepo};
use async_trait::async_trait;
use sqlx::{mysql::MySql, Pool};

pub struct DataStore {
    pub pool: Pool<MySql>,
}

#[async_trait]
impl UserRepo for DataStore {
    async fn create(&self, new_user: abi::User) -> Result<(), UserError> {
        let sql = "INSERT INTO `users` ( `username`, `avatar_url`, `email`, `status` ) VALUES (?, ?, ?)";
        sqlx::query(sql)
            .bind(new_user.username)
            .bind(new_user.avatar_url)
            .bind(new_user.email)
            .bind(new_user.status)
            .execute(&(self.pool))
            .await?;
        Ok(())
    }

    async fn update(&self, user: abi::User) -> Result<(), UserError> {
        todo!()
    }

    async fn delete(&self, user_id: UserId) -> Result<(), UserError> {
        let sql = "DELETE FROM `users` WHERE `id` = ?";
        sqlx::query(sql).bind(user_id).execute(&(self.pool)).await?;
        Ok(())
    }

    async fn get(&self, user_id: UserId) -> Result<abi::User, UserError> {
        todo!()
    }

    async fn list() -> Result<Vec<abi::User>, UserError> {
        todo!()
    }
}

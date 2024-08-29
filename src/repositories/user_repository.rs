use std::convert::TryFrom;
use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;
use crate::errors::{ApiError};

use crate::models::user::{User, UserInsert, UserRegister, UserRepository};

#[async_trait]
impl UserRepository for PgConnection {
    async fn create_user(
        &mut self,
        user_register: &UserRegister,
    ) -> Result<User, ApiError> {
        let user = UserInsert::try_from(user_register)?;
        let registered_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (email, otp_secret)
            VALUES ($1, $2)
            RETURNING *
            "#,
            user.email,
            user.otp_secret
        )
            .fetch_one(self)
            .await?;

        Ok(registered_user)
    }

    async fn find_user_by_email(&mut self, email: &str) -> Result<User, ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE email = $1
            "#,
            email
        )
            .fetch_one(self)
            .await?;

        Ok(user)
    }

    async fn find_user_by_id(&mut self, id: Uuid) -> Result<User, ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "user"
            WHERE id = $1
            "#,
            id
        )
            .fetch_one(self)
            .await?;

        Ok(user)
    }
}
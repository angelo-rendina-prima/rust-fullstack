#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    id: uuid::Uuid,
    email: String,
}

impl User {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub async fn get_by_credentials(
        executor: impl sqlx::PgExecutor<'_>,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
        SELECT
            id, email
        FROM users
        WHERE email = $1 AND password = $2
            "#,
            email,
            password,
        )
        .fetch_optional(executor)
        .await
    }
}

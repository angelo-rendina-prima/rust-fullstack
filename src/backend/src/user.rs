#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}

impl User {
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Todo {
    id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    message: String,
}

impl Todo {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    pub fn completed_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.completed_at
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub async fn get_all(executor: impl sqlx::PgExecutor<'_>) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"
        SELECT
            id, created_at, completed_at, message
        FROM todos
            "#,
        )
        .fetch_all(executor)
        .await
    }

    pub async fn get_by_id(
        executor: impl sqlx::PgExecutor<'_>,
        id: &uuid::Uuid,
    ) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"
        SELECT
            id, created_at, completed_at, message
        FROM todos
        WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(executor)
        .await
    }

    pub async fn insert(
        executor: impl sqlx::PgExecutor<'_>,
        todo: &Todo,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        INSERT INTO todos
            (id, created_at, completed_at, message)
        VALUES($1, $2, $3, $4)
            "#,
        )
        .bind(todo.id())
        .bind(todo.created_at())
        .bind(todo.completed_at())
        .bind(todo.message())
        .execute(executor)
        .await
        .map(|_| ())
    }

    pub async fn update(
        executor: impl sqlx::PgExecutor<'_>,
        todo: &Todo,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        UPDATE todos
        SET id = $1,
            created_at = $2,
            completed_at = $3,
            message = $4
        WHERE id = $1
            "#,
        )
        .bind(todo.id())
        .bind(todo.created_at())
        .bind(todo.completed_at())
        .bind(todo.message())
        .execute(executor)
        .await
        .map(|_| ())
    }
}

pub async fn get_all(app: actix_web::web::Data<crate::App>) -> actix_web::HttpResponse {
    match Todo::get_all(app.pool()).await {
        Ok(posts) => actix_web::HttpResponse::Ok().json(posts),
        Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewTodoPayload {
    message: String,
}

pub async fn create(
    app: actix_web::web::Data<crate::App>,
    payload: actix_web::web::Json<NewTodoPayload>,
) -> actix_web::HttpResponse {
    let id = uuid::Uuid::new_v4();
    let todo = Todo {
        id,
        created_at: chrono::Utc::now(),
        completed_at: None,
        message: payload.into_inner().message,
    };
    match Todo::insert(app.pool(), &todo).await {
        Ok(_) => actix_web::HttpResponse::Ok().body(format!("{id}")),
        Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateTodoPayload {
    id: uuid::Uuid,
    done: bool,
    message: String,
}

pub async fn resolve(
    app: actix_web::web::Data<crate::App>,
    payload: actix_web::web::Json<UpdateTodoPayload>,
) -> actix_web::HttpResponse {
    let payload = payload.into_inner();
    match Todo::get_by_id(app.pool(), &payload.id).await {
        Ok(Some(mut todo)) => {
            todo.completed_at = match payload.done {
                true => Some(chrono::Utc::now()),
                false => None,
            };
            todo.message = payload.message;
            match Todo::update(app.pool(), &todo).await {
                Ok(_) => actix_web::HttpResponse::Ok().finish(),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
            }
        }
        _ => actix_web::HttpResponse::NotFound().finish(),
    }
}

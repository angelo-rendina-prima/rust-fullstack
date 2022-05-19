#[derive(serde::Serialize, serde::Deserialize)]
pub struct Todo {
    id: uuid::Uuid,
    author_id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    message: String,
}

impl Todo {
    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    pub fn author_id(&self) -> &uuid::Uuid {
        &self.author_id
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

    pub async fn get_by_author_id(
        executor: impl sqlx::PgExecutor<'_>,
        author_id: &uuid::Uuid,
    ) -> Result<Vec<Todo>, sqlx::Error> {
        sqlx::query_as!(
            Todo,
            r#"
        SELECT
            id, author_id, created_at, completed_at, message
        FROM todos
        WHERE author_id = $1
            "#,
            author_id,
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
            id, author_id, created_at, completed_at, message
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
            (id, author_id, created_at, completed_at, message)
        VALUES($1, $2, $3, $4, $5)
            "#,
        )
        .bind(todo.id())
        .bind(todo.author_id())
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
            author_id = $2,
            created_at = $3,
            completed_at = $4,
            message = $5
        WHERE id = $1
            "#,
        )
        .bind(todo.id())
        .bind(todo.author_id())
        .bind(todo.created_at())
        .bind(todo.completed_at())
        .bind(todo.message())
        .execute(executor)
        .await
        .map(|_| ())
    }
}

pub async fn get_all_for_active_user(
    app: actix_web::web::Data<crate::App>,
    session: actix_session::Session,
) -> actix_web::HttpResponse {
    match session.get::<crate::user::User>("user") {
        Ok(Some(user)) => match Todo::get_by_author_id(app.pool(), user.id()).await {
            Ok(posts) => actix_web::HttpResponse::Ok().json(posts),
            Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
        },
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewTodoPayload {
    message: String,
}

pub async fn create(
    app: actix_web::web::Data<crate::App>,
    session: actix_session::Session,
    payload: actix_web::web::Json<NewTodoPayload>,
) -> actix_web::HttpResponse {
    match session.get::<crate::user::User>("user") {
        Ok(Some(user)) => {
            let id = uuid::Uuid::new_v4();
            let todo = Todo {
                id,
                author_id: *user.id(),
                created_at: chrono::Utc::now(),
                completed_at: None,
                message: payload.into_inner().message,
            };
            match Todo::insert(app.pool(), &todo).await {
                Ok(_) => actix_web::HttpResponse::Ok().body(format!("{id}")),
                Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
            }
        }
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MarkTodoPayload {
    id: uuid::Uuid,
    done: bool,
}

pub async fn resolve(
    app: actix_web::web::Data<crate::App>,
    session: actix_session::Session,
    payload: actix_web::web::Json<MarkTodoPayload>,
) -> actix_web::HttpResponse {
    let payload = payload.into_inner();
    match session.get::<crate::user::User>("user") {
        Ok(Some(_)) => match Todo::get_by_id(app.pool(), &payload.id).await {
            Ok(Some(todo)) => {
                let todo = Todo {
                    completed_at: match payload.done {
                        true => Some(chrono::Utc::now()),
                        false => None,
                    },
                    ..todo
                };
                match Todo::update(app.pool(), &todo).await {
                    Ok(_) => actix_web::HttpResponse::Ok().finish(),
                    Err(_) => actix_web::HttpResponse::InternalServerError().finish(),
                }
            }
            _ => actix_web::HttpResponse::NotFound().finish(),
        },
        Err(x) => actix_web::HttpResponse::BadRequest().body(format!("{x:?}")),
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

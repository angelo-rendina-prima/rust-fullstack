pub async fn whoami(session: actix_session::Session) -> actix_web::HttpResponse {
    match session.get::<crate::user::User>("user") {
        Ok(Some(user)) => actix_web::HttpResponse::Ok().json(user),
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginCredentials {
    email: String,
    password: String,
}

pub async fn login(
    app: actix_web::web::Data<crate::App>,
    session: actix_session::Session,
    credentials: actix_web::web::Json<LoginCredentials>,
) -> actix_web::HttpResponse {
    let LoginCredentials { email, password } = credentials.into_inner();
    let auth_result = crate::user::User::get_by_credentials(app.pool(), &email, &password).await;
    match auth_result {
        Ok(Some(user)) => {
            session
                .insert("user", user)
                .expect("Failed writing to session");
            actix_web::HttpResponse::Ok().finish()
        }
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

pub async fn logout(session: actix_session::Session) -> actix_web::HttpResponse {
    session.remove("user");
    actix_web::HttpResponse::Ok().finish()
}

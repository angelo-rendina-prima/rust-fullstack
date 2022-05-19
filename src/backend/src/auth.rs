pub async fn whoami(session: actix_session::Session) -> actix_web::HttpResponse {
    match session.get::<crate::user::User>("user") {
        Ok(Some(user)) => {
            actix_web::HttpResponse::Ok().body(format!("{}\n{}", &user.email, &user.id,))
        }
        _ => actix_web::HttpResponse::Unauthorized().finish(),
    }
}

pub async fn login(
    app: actix_web::web::Data<crate::App>,
    session: actix_session::Session,
    path: actix_web::web::Path<(String, String)>,
) -> actix_web::HttpResponse {
    let (email, password) = path.into_inner();
    let auth_result = crate::user::User::get_by_credentials(&app.pool, &email, &password).await;
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

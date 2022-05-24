use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: uuid::Uuid,
    email: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct AuthCredentials {
    email: String,
    password: String,
}

pub struct Model {
    state: State,
    credentials: AuthCredentials,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            state: State::Credentials,
            credentials: AuthCredentials::default(),
        }
    }
}

impl Model {
    fn can_edit_credentials(&self) -> bool {
        !matches!(&self.state, State::Working)
    }
}

enum State {
    Credentials,
    Working,
}

pub enum Msg {
    EmailChanged(String),
    PasswordChanged(String),
    LogIn,
    AuthFailed,
    Authenticated(User),
}

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::EmailChanged(email) => {
            model.credentials.email = email;
        }
        Msg::PasswordChanged(password) => {
            model.credentials.password = password;
        }
        Msg::LogIn => {
            let credentials = model.credentials.clone();
            model.state = State::Working;
            orders.perform_cmd(async move {
                // POST login
                let request = Request::new("http://localhost:3000/auth/login")
                    .method(Method::Post)
                    .json(&credentials);
                let request = match request {
                    Ok(request) => request,
                    Err(_) => {
                        return Msg::AuthFailed;
                    }
                };

                let response = request.fetch().await;
                let response = match response {
                    Ok(response) => response,
                    Err(_) => {
                        return Msg::AuthFailed;
                    }
                };
                if response.check_status().is_err() {
                    return Msg::AuthFailed;
                }

                // GET whoami
                let request = Request::new("http://localhost:3000/auth/")
                    .method(Method::Get)
                    .credentials(web_sys::RequestCredentials::Include);

                let response = request.fetch().await;
                let response = match response {
                    Ok(response) => response,
                    Err(_) => {
                        return Msg::AuthFailed;
                    }
                };
                let user = match response.json::<User>().await {
                    Ok(user) => user,
                    Err(_) => {
                        return Msg::AuthFailed;
                    }
                };

                Msg::Authenticated(user)
            });
        }
        Msg::AuthFailed => {
            model.state = State::Credentials;
        }
        Msg::Authenticated(_) => {
            model.state = State::Credentials;
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
        h1!("Login"),
        div!(
            label!("Email"),
            input!(
                attrs! {
                    // At::Disabled => !model.can_edit_credentials(),
                },
                input_ev(Ev::Input, move |email| Msg::EmailChanged(email))
            ),
        ),
        div!(
            label!("Password"),
            input!(
                attrs! {
                    // At::Disabled => !model.can_edit_credentials(),
                },
                input_ev(Ev::Input, move |password| Msg::PasswordChanged(password))
            ),
        ),
        button!(
            "Admin",
            attrs! {
                // At::Disabled => !model.can_edit_credentials(),
            },
            ev(Ev::Click, |_| Msg::LogIn)
        )
    ]
}

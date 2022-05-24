mod auth;

use seed::{prelude::*, *};

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

// #[derive(Serialize, Deserialize)]
// pub struct Todo {
//     id: uuid::Uuid,
//     author_id: uuid::Uuid,
//     created_at: chrono::DateTime<chrono::Utc>,
//     completed_at: Option<chrono::DateTime<chrono::Utc>>,
//     message: String,
// }

struct Model {
    state: State,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            state: State::Auth(auth::Model::default()),
        }
    }
}

enum State {
    Auth(auth::Model),
}

enum Msg {
    Auth(auth::Msg),
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match (msg, &mut model.state) {
        (Msg::Auth(auth_msg), State::Auth(auth_model)) => {
            auth::update(auth_msg, auth_model, &mut orders.proxy(Msg::Auth))
        },
    }
}

fn view(model: &Model) -> Node<Msg> {
    match &model.state {
        State::Auth(auth_model) => auth::view(auth_model).map_msg(Msg::Auth),
    }
}

mod todo;

use seed::{prelude::*, *};
use todo::Todo;

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

struct Model {
    working: bool,
    todos: Vec<Todo>,
    adding: String,
}

enum Msg {
    Load,
    Loaded(Vec<Todo>),
    Toggle(usize),
    Write(String),
    Add,
    Delete(usize),
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::Load });

    Model {
        working: true,
        todos: vec![],
        adding: String::new(),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Load => {
            model.working = true;
            orders.perform_cmd(async {
                let todos = todo::get_all().await;
                Msg::Loaded(todos)
            });
        }
        Msg::Loaded(todos) => {
            model.todos = todos;
            model.working = false;
        }
        Msg::Toggle(index) => {
            let todo = &model.todos[index];
            let payload = todo::UpdateTodoPayload {
                id: *todo.id(),
                message: todo.message().to_string(),
                done: todo.completed_at().is_none(),
            };
            model.working = true;
            orders.perform_cmd(async move {
                todo::update(&payload).await;
                Msg::Load
            });
        }
        Msg::Write(text) => {
            model.adding = text;
        }
        Msg::Add => {
            model.working = true;
            let payload = todo::NewTodoPayload {
                message: std::mem::take(&mut model.adding),
            };
            orders.perform_cmd(async move {
                todo::new(&payload).await;
                Msg::Load
            });
        }
        Msg::Delete(index) => {
            model.working = true;
            let payload = todo::DeleteTodoPayload {
                id: *model.todos[index].id(),
            };
            orders.perform_cmd(async move {
                todo::delete(&payload).await;
                Msg::Load
            });
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div!(
        h1!("Todo"),
        div!(
            textarea!(
                attrs! {
                    At::Disabled => model.working.as_at_value(),
                    At::Value => model.adding,
                },
                IF!(not(model.working) => input_ev(Ev::Input, |text| Msg::Write(text))),
            ),
            button!(
                "Add",
                attrs! {
                    At::Disabled => {
                        model.working || model.adding.is_empty()
                    }.as_at_value(),
                },
                IF!(not(model.working) => ev(Ev::Click, |_| Msg::Add)),
            ),
        ),
        model
            .todos
            .iter()
            .enumerate()
            .rev()
            .map(|(index, todo)| view_todo(model, index, todo)),
    )
}

fn view_todo(model: &Model, index: usize, todo: &Todo) -> Node<Msg> {
    div!(
        C!["todo"],
        button!(
            "X",
            C!["todo__x"],
            attrs! {
                At::Disabled => {
                    model.working
                }.as_at_value(),
            },
            IF!(not(model.working) => ev(Ev::Click, move |_| Msg::Delete(index))),
        ),
        div!(C!["todo__id"], format!("{}", todo.id()),),
        div!(
            C!["todo__times"],
            div!(
                "Started:",
                C!["todo__times__start"],
                format!("{}", todo.created_at()),
            ),
            div!(
                "Completed:",
                C!["todo__times__end"],
                input!(
                    attrs! {
                        At::Type => "checkbox",
                        At::Checked => todo.completed_at().is_some().as_at_value(),
                        At::Disabled => model.working.as_at_value(),
                    },
                    ev(Ev::Click, move |_| Msg::Toggle(index)),
                ),
                span!(todo
                    .completed_at()
                    .map(|d| format!("{d}"))
                    .unwrap_or("---".to_string())),
            ),
        ),
        div!(C!["todo__message"], format!("{}", todo.message()),),
    )
}

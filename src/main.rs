use axum::{
    extract::{Query, State},
    response::{Html, Redirect},
    routing::get,
    Router,
};
use minijinja::{context, path_loader, Environment};
use serde::Deserialize;

#[derive(Clone)]
struct AppState {
    env: Environment<'static>,
}

#[derive(Deserialize)]
struct QueryString {
    q: String,
}

async fn get_contacts(State(state): State<AppState>, query: Query<QueryString>) -> Html<String> {
    let tmpl = state.env.get_template("index.html").unwrap();
    let r = tmpl.render(context! {q  => query.q}).unwrap();
    Html(r)
}

#[tokio::main]
async fn main() {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let state = AppState { env };

    let app = Router::new()
        .route("/", get(|| async { Redirect::permanent("/contacts") }))
        .route("/contacts", get(get_contacts))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

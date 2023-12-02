use std::io::Error;
use std::net::TcpListener;

use crate::routes::contacts;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use minijinja::{path_loader, Environment};

struct AppState {
    env: Environment<'static>,
}

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let state = web::Data::new(AppState { env });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::redirect("/", "/contacts"))
            .route("/contacts", web::get().to(contacts))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

use actix_files::Files;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::io::Error;
use std::net::TcpListener;

use crate::api;

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(Files::new("/public", "./public"))
            .service(web::redirect("/", "/contacts"))
            .service(api::contact::contacts)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

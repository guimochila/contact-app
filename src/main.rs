use contact_app::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("127.0.0.1:3000");
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}

use contact_app::repository::db;
use contact_app::settings::Settings;
use contact_app::startup::run;
use contact_app::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("htmx-rust".into(), "info".into());
    init_subscriber(subscriber);

    db::DataBase::new("sqlite://database/db.sql".to_string()).await;

    let settings = Settings::new().unwrap();
    let address = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(&address)?;

    info!("🚀 server is running on {}", address);

    run(listener)?.await
}

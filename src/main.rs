use zerotoprod::configuration::get_configuration;
use zerotoprod::startup::run;
use zerotoprod::telemetry::{get_subscriber, init_subscriber,get_dual_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (subscriber,_guard)=get_dual_subscriber(
        "zerotoprod".into(),
        "info".into(),
        "logs",
        "zerotoprod"
    );
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}

use zerotoprod::configuration::get_configuration;
use zerotoprod::startup::run;
use zerotoprod::telemetry::{init_subscriber,get_dual_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (subscriber,_guard)=get_dual_subscriber(
        "zerotoprod".into(),
        "info".into(),
        "logs",
        "zerotoprod"
    );
    init_subscriber(subscriber);
    let configuration = get_configuration()?;
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await?;
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}

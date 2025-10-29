use zerotoprod::configuration::get_configuration;
use zerotoprod::startup::run;
use zerotoprod::telemetry::{init_subscriber,get_dual_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;
use tracing::error;


#[tokio::main]
async fn main() -> std::io::Result<()>{
    let (subscriber,_guard)=get_dual_subscriber(
        "zerotoprod".into(),
        "info".into(),
        "logs",
        "zerotoprod"
    ).unwrap_or_else(|e|{
        eprintln!("failed to create telemetry subscriber:{}",e);
        panic!("Telemetry setup is required for startup: {}",e);
    });
    init_subscriber(subscriber).unwrap_or_else(|e| {
        eprintln!("Failed to initialize telemetry: {}", e);
        panic!("Telemetry initialization is required for startup: {}", e);
    });
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    
    run(listener, connection_pool).unwrap_or_else(|e| {
        error!("Failed to start server: {}", e);
        panic!("Server startup failed: {}", e);
    }).await?;
    
    Ok(())
}

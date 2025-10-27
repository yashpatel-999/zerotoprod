use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer,JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt,EnvFilter,Registry};
use tracing_appender::{non_blocking,rolling};
use tracing_subscriber::fmt;


pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync + 'static) {
    LogTracer::init().ok();
    set_global_default(subscriber).ok();

pub fn get_dual_subscriber(
    name:String,
    env_filter:String,
    log_directory:&str,
    file_prefix:&str,
)->(impl Subscriber + Send +Sync, tracing_appender::non_blocking::WorkerGuard){
    let env_filter=EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    
    let file_appender=rolling::daily(log_directory,file_prefix);
    let (file_writer,guard)=non_blocking(file_appender);

    let console_layer=BunyanFormattingLayer::new(name.clone(),std::io::stdout);

    let file_layer=fmt::layer()
        .with_writer(file_writer)
        .with_target(false)
        .with_ansi(false);
    
    let subscriber=Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(console_layer)
        .with(file_layer);

    (subscriber,guard)
}
use actix_web::{HttpResponse,ResponseError};
use actix_web::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum SubscribeError{
    ValidationError(String),
    DatabaseError(sqlx::Error),
    UnexpectedError(anyhow::Error),
}

#[derive(Debug)]
pub enum StartupError{
    ServerBind(std::io::Error),
    InvalidConfiguration(String),
    ResourceAllocation(String),
}

#[derive(Debug)]
pub enum TelemetryError{
    LogTracerInit(tracing_log::log::SetLoggerError),
    SubscriberInit(tracing::subscriber::SetGlobalDefaultError),
    EnvFilterParse(String),
    FileAppenderError(String),
    InvalidConfiguration(String),
    IoError(std::io::Error),
}

impl fmt::Display for SubscribeError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            SubscribeError::ValidationError(e)=>write!(f,"Validation error: {}",e),
            SubscribeError::DatabaseError(e)=>write!(f,"Database error : {}",e),
            SubscribeError::UnexpectedError(e)=>write!(f,"Unexpected error: {}",e),
        }
    }
}

impl ResponseError for SubscribeError{
    fn status_code(&self)->StatusCode{
        match self{
            SubscribeError::ValidationError(_)=>StatusCode::BAD_REQUEST,
            SubscribeError::DatabaseError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            SubscribeError::UnexpectedError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self)->HttpResponse{
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({
                "error": self.to_string()
            }))
    }
}
impl From<sqlx::Error> for SubscribeError{
    fn from(e: sqlx::Error) -> Self{
        SubscribeError::DatabaseError(e)
    }
}
impl From<anyhow::Error> for SubscribeError{
    fn from(e: anyhow::Error) -> Self{
        SubscribeError::UnexpectedError(e)
    }
}

impl fmt::Display for StartupError{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            StartupError::ServerBind(err)=>{write!(f,"Failed to bind server to listener {}",err)}
            StartupError::InvalidConfiguration(msg)=>{write!(f,"Invalid server configuration: {}",msg)}
            StartupError::ResourceAllocation(msg)=>{write!(f,"Resource allocation failed: {}",msg)}
        }
    }
}

impl std::error::Error for StartupError{
    fn source(&self)->Option<&(dyn std::error::Error + 'static)>{
        match self{
            StartupError::ServerBind(err)=>Some(err),
            _=>None,
        }
    }
}

impl From<std::io::Error> for StartupError{
    fn from(err: std::io::Error)->Self{
        StartupError::ServerBind(err)
    }
}

impl fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelemetryError::LogTracerInit(err) => {
                write!(f, "Failed to initialize log tracer: {}", err)
            }
            TelemetryError::SubscriberInit(err) => {
                write!(f, "Failed to set global tracing subscriber: {}", err)
            }
            TelemetryError::EnvFilterParse(msg) => {
                write!(f, "Failed to parse environment filter: {}", msg)
            }
            TelemetryError::FileAppenderError(msg) => {
                write!(f, "Failed to create file appender: {}", msg)
            }
            TelemetryError::InvalidConfiguration(msg) => {
                write!(f, "Invalid telemetry configuration: {}", msg)
            }
            TelemetryError::IoError(err) => {
                write!(f, "IO error in telemetry setup: {}", err)
            }
        }
    }
}

impl std::error::Error for TelemetryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TelemetryError::LogTracerInit(err) => Some(err),
            TelemetryError::SubscriberInit(err) => Some(err),
            TelemetryError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<tracing_log::log::SetLoggerError> for TelemetryError{
    fn from(err:tracing_log::log::SetLoggerError)->Self{
        TelemetryError::LogTracerInit(err)
    }
}

impl From<tracing::subscriber::SetGlobalDefaultError> for TelemetryError{
    fn from(err: tracing::subscriber::SetGlobalDefaultError)->Self{
        TelemetryError::SubscriberInit(err)
    }
}

impl From<std::io::Error> for TelemetryError{
    fn from(err: std::io::Error)->Self{
        TelemetryError::IoError(err)
    }
}
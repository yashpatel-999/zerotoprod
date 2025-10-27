use actix_web::{HttpResponse,ResponseError};
use actix_web::http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum SubscribeError{
    ValidationError(String),
    DatabaseError(sqlx::Error),
    UnexpectedError(anyhow::Error),
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
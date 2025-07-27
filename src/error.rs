use axum::{http::{StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LogicFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    TicketDeleteFailIdNotFound {id: u64},
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("->> {:<12} - {self:?}", "INTO_RES");
        
        let mut  response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        
        #[allow(unreachable_patterns)]
        match self {
            Self::LogicFail => (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS),
            
            Self::AuthFailNoAuthTokenCookie 
            |  Self::AuthFailTokenWrongFormat
            |  Self::AuthFailCtxNotInRequestExt => 
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),
            
            Self::TicketDeleteFailIdNotFound { .. } => (StatusCode::BAD_REQUEST, ClientError::SERVICE_ERROR),
            
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
            },
        }
    }
    
}

#[derive(strum_macros::AsRefStr, Debug)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR
}
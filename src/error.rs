use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

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
        println!("->> {:<12} - {self:?}", "INTO_RES");
        
        (StatusCode::INTERNAL_SERVER_ERROR, "MY_CLIENT_ERROR").into_response()
    }
    
}
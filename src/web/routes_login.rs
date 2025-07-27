use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;
use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    debug!("->> {:<12} - api_login", "HANDLER");
        
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LogicFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    let body: Json<Value> = Json(json!({
        "result": {
            "success" : true
        }}
    ));
    
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

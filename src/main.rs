#[allow(unused)]

use crate::web::routes_login::routes;


pub use self::error::{Error, Result};
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_all = Router::new()
		.merge(routes_hello())
		.merge(routes())
		.layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new())
		.fallback_service(get_service(ServeDir::new("./src")).handle_error(|err| async move {
				(
					axum::http::StatusCode::INTERNAL_SERVER_ERROR,
					format!("Unhandled internal error: {}", err),
				)
        }));

	// region:    --- Start Server
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
	println!("--> {:<12} - main responce", "RES_MAPPER");

	res
}

fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/{name}", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}

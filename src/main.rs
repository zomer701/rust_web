#[allow(unused)]

mod config;
mod ctx;
mod error;
mod logs;
mod web;
mod model;

pub use self::error::{Error, Result};
pub use config::config;

use crate::ctx::{Ctx, RequestData};
use crate::model::model::ModelController;
use crate::web::{routes_login, routes_static};

use axum::extract::{Path, Query};
use axum::http::{Request};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get};
use axum::{middleware, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;


#[tokio::main]
async fn main() -> Result<()> {

	tracing_subscriber::fmt()
		.without_time()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let mc = ModelController::new().await?;

	// let routes_apis = web::routes_tickets::routes(mc.clone())
	// 	.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

	let routes_all = Router::new()
		.merge(routes_hello())
		.merge(routes_login::routes())
		//.nest("/api", routes_apis)
		.layer(middleware::from_fn(capture_request_data))
		.layer(middleware::map_response(main_response_mapper))
		.layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
	
	info!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
	info!("->> {:<12} - main responce", "RES_MAPPER");

	 let default_request_data = RequestData::default();
	 let request_data = res.extensions().get::<RequestData>().unwrap_or(&default_request_data);

	let ctx = request_data.ctx.clone();
	let uri = request_data.uri.clone();
	let req_method = request_data.method.clone();

	let uuid = uuid::Uuid::new_v4();

	let server_error = res.extensions().get::<Error>();
	let client_status_error = server_error
		.map(|se| se.client_status_and_error());

	let error_response = client_status_error.as_ref()
		.map(|(status, client_error)| {
			let client_error_body = json!({
				"error": {
					"type": client_error.as_ref(),
					"request_uuid": uuid.to_string()
				}
			});
			info!("->> {:<12} - main error response", "RES_MAPPER");
			info!("->> {:#?}", client_error_body);

			(*status, Json(client_error_body)).into_response()
		});

	info!("->> {:<12} - server_error response", "RES_MAPPER");
	let client_error = client_status_error.unzip().1;
	logs::log_request(
		uuid,
		req_method,
		uri,
		ctx,
		server_error,
		client_error
	).await.ok();

	error_response.unwrap_or(res)
}

use axum::body::Body;
use tracing::info;
use tracing_subscriber::EnvFilter;

async fn capture_request_data(
	mut request: Request<Body>,
	next: Next,
) -> Response {
	// Extract values from the request
	let method = request.method().clone();
	let uri = request.uri().clone();
	let ctx = request.extensions().get::<Ctx>().cloned(); // Get from previous middleware
	if ctx.is_none() {
		info!("->> {:<12} - No Ctx found in request extensions", "MW_CAPTURE");
	}
	// Store values in response extensions for map_response
	request.extensions_mut().insert(RequestData { method, uri, ctx });
	
	next.run(request).await
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
	info!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	info!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}

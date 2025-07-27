use crate::{ctx::Ctx, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;
use chrono::Utc;
use tracing::debug;

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Option<Ctx>,
	service_error: Option<&Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: Utc::now().to_rfc3339(),
		user_id: ctx.map(|c| c.user_id()),
		req_path: uri.path().to_string(),
		req_method: req_method.to_string(),
		client_error_type: client_error.map(|e| e.as_ref().to_string()),
		error_type: service_error.map(|e| e.as_ref().to_string()),
		error_data: service_error
			.and_then(|e| serde_json::to_value(e).ok())
			.and_then(|mut v| v.get_mut("data").map(|v| v.take())),
	};

	debug!("->> {:<12} - log_request", "LOGGER");
	debug!("{:#?}", log_line);

	// Here you would typically write the log_line to a file or logging system.
	Ok(())
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
struct RequestLogLine {
	uuid: String,      // uuid string formatted
	timestamp: String, // (should be iso8601)

	// -- User and context attributes.
	user_id: Option<u64>,

	// -- http request attributes.
	req_path: String,
	req_method: String,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}
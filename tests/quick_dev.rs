#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_http_requests() -> Result<()> {
	let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

	//hc.do_get("/hello2/Mike").await?.print().await?;

	//hc.do_get("/src/main.rs").await?.print().await?;

    //hc.do_get("/api/tickets").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    ).await?.print().await?;

    hc.do_post("/api/tickets", 
        json!({
            "title": "addd"
        })
    ).await?.print().await?;

     
    hc.do_delete("/api/tickets/1").await?.print().await?;

	Ok(())
}

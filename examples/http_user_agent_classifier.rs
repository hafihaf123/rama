//! An example to showcase how you can classify the User-Agent of incoming requests.
//!
//! # Run the example
//!
//! ```sh
//! cargo run --example http_user_agent_classifier
//! ```
//!
//! # Expected output
//!
//! The server will start and listen on `:40015`. You can use `curl` to interact with the service:
//!
//! ```sh
//! curl -v http://127.0.0.1:40015
//! ```
//!
//! You should see a response with `HTTP/1.1 200 OK` and a JSON body with the user agent info exposed by Rama.

use std::convert::Infallible;

use rama::{
    http::{response::Json, server::HttpServer, IntoResponse, Request, Response},
    rt::Executor,
    service::{Context, ServiceBuilder},
    ua::{UserAgent, UserAgentClassifierLayer},
};
use serde_json::json;

#[tokio::main]
async fn main() {
    let exec = Executor::default();
    HttpServer::auto(exec)
        .listen(
            "127.0.0.1:40015",
            ServiceBuilder::new()
                .layer(UserAgentClassifierLayer::new())
                .service_fn(handle),
        )
        .await
        .unwrap();
}

async fn handle(ctx: Context<()>, _req: Request) -> Result<Response, Infallible> {
    let ua: &UserAgent = ctx.get().unwrap();
    Ok(Json(json!({
        "ua": ua.header_str(),
        "kind": ua.info().map(|info| info.kind.to_string()),
        "version": ua.info().and_then(|info| info.version),
        "platform": ua.platform().map(|p| p.to_string()),
    }))
    .into_response())
}
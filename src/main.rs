use axum::extract::Query;
use axum::http::StatusCode;
use axum::{response::Json, routing::get, Router};
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env::set_var;

#[derive(Deserialize, Serialize)]
struct Params {
    name: Option<String>,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn root(Query(params): Query<Params>) -> Json<Value> {
    let who = params.name.unwrap_or(String::from("World"));
    Json(json!({ "msg":
        format!("Hello, {who}, this is an AWS Lambda response from an axum server")
    }))
}

async fn health_check() -> (StatusCode, String) {
    (StatusCode::OK, "Healthy!".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
    tracing::init_default_subscriber();
    let app = Router::new()
        .route("/", get(root))
        .route("/health/", get(health_check));

    run(app).await
}

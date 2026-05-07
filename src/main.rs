use anyhow::bail;
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::post,
};
use tracing::{error, info, instrument};
use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_line_number(true)
        .init();
}

#[tokio::main]
#[instrument]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();

    info!("Setting up NAAS service, your friendly null check companion");

    let app = Router::new().route("/", post(check_null));

    info!("Binding configured port");

    let Ok(listener) = tokio::net::TcpListener::bind("0.0.0.0:3000").await else {
        error!("Error when trying to set up tcp listener");
        bail!("Error creating tcp listener");
    };

    info!("Serving requests, null checks are important");

    let Ok(()) = axum::serve(listener, app).await else {
        error!("Error when trying to serve requests");
        bail!("Error listening for requests!");
    };

    info!("Done with null checking for now, bye");

    Ok(())
}

#[instrument]
async fn check_null(body: String) -> Response {
    let trimmed = body.trim().to_lowercase();

    if trimmed.is_empty() || trimmed == "null" {
        info!("Request is null");
        return "NULL!".into_response();
    }

    info!("Request is not null");
    "NOT NULL!".into_response()
}

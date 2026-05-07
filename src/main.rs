use anyhow::bail;
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing::post,
};
use envconfig::Envconfig;
use tracing::{error, info, instrument};
use tracing_subscriber::{EnvFilter, fmt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_line_number(true)
        .init();
}

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "PORT", default = "3000")]
    port: String,
}

#[tokio::main]
#[instrument]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();

    info!("Reading config from env");

    let Ok(config) = Config::init_from_env() else {
        error!("Error when reading config from env");
        bail!("Error when reading config from env");
    };

    info!("Found config: {:?}", config);

    info!("Setting up NAAS service, your friendly null check companion");

    let app = Router::new()
        .route("/check", post(check_null))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    info!("Binding configured port");

    let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await
    else {
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

#[derive(OpenApi)]
#[openapi(
    paths(check_null),
    tags(
        (name = "api", description = "Null check endpoints")
    )
)]
struct ApiDoc;

#[instrument]
#[utoipa::path(
    post,
    path = "/check",
    request_body = Option<String>,
    responses(
        (status = 200, description = "Null check", body = String)
    )
)]
async fn check_null(body: String) -> Response {
    let trimmed = body.trim().to_lowercase();

    if trimmed.is_empty() || trimmed == "null" {
        info!("Request is null");
        return "NULL!".into_response();
    }

    info!("Request is not null");
    "NOT NULL!".into_response()
}

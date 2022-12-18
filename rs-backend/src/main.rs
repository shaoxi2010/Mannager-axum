mod router;
use anyhow::{Ok, Result};
use axum::{
    http::StatusCode,
    routing::{self, Router},
};
use tower_http::{services::ServeDir, trace::TraceLayer};
use router::init_resource_router;
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:3000".parse()?;
    tracing_subscriber::fmt::init();

    let vue_app = Router::new()
        .fallback_service(
            routing::get_service(ServeDir::new("../dist"))
                .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR }),
        )
        .nest("/devices", init_resource_router())
        .layer(TraceLayer::new_for_http());
    tracing::debug!("Start Server at {:?}", addr);
    axum::Server::bind(&addr)
        .serve(vue_app.into_make_service())
        .await?;
    Ok(())
}

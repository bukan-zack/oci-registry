use anyhow::Result;
use axum::Router;

mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .nest("/v2", api::v2::routing());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    Ok(axum::serve(listener, app).await?)
}

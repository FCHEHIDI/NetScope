use anyhow::Context;
use api_server::build_router;
use config::AppConfig;
use observability::init_tracing;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = AppConfig::default();
    config.validate().context("invalid app configuration")?;

    let bind_addr = config.service.bind_addr.clone();
    let router = build_router(config);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .with_context(|| format!("failed to bind to {bind_addr}"))?;

    info!(%bind_addr, "starting NetScope app");
    axum::serve(listener, router).await.context("axum server exited")?;
    Ok(())
}

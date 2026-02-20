use api_server::build_router;
use config::AppConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::default();
    let router = build_router(config.clone());
    let listener = tokio::net::TcpListener::bind(&config.service.bind_addr).await?;

    axum::serve(listener, router).await?;
    Ok(())
}

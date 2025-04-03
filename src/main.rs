use mozu::{HttpServer, HttpServerConfig, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    tracing::info!("Connect postgres...");
    let postgres_url = std::env::var("DATABASE_URL")?;
    let pg = Postgres::new(&postgres_url).await?;
    tracing::info!("Connected to postgres");

    tracing::info!("Starting HTTP server...");
    let server_config = HttpServerConfig::builder()
        .host_url("http://localhost:3000".to_string())
        .port(3000)
        .build();
    let server = HttpServer::new(server_config, pg);
    server.run().await?;
    tracing::info!("HTTP server stopped");

    Ok(())
}

#[tracing::instrument]
async fn handler() -> &'static str {
    "Hello, world!"
}

fn init_tracing() {
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        };
        level.into()
    });

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true);

    #[cfg(debug_assertions)]
    let fmt_layer = fmt_layer.with_ansi(true).pretty();
    #[cfg(not(debug_assertions))]
    let fmt_layer = fmt_layer.json();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();
}

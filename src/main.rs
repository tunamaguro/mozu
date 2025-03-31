#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    bootstrap().await?;

    Ok(())
}

async fn bootstrap() -> anyhow::Result<()> {
    use axum::routing;
    use std::net::Ipv4Addr;
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;

    let router = axum::Router::new()
        .route("/", routing::get(handler))
        .layer(TraceLayer::new_for_http());

    let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);
    axum::serve(listener, router).await?;

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
            "trace"
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

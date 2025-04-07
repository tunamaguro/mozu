pub(crate) mod accounts;
pub(crate) mod ap;
pub(crate) mod state;
pub(crate) mod utils;
pub(crate) mod well_known;
use state::AppRegistryExt as _;
use tokio::signal;
use typed_builder::TypedBuilder;

use crate::{domain::hosturl::HostUrl, infrastructure::postgres::Postgres};

#[derive(Debug, TypedBuilder)]
pub struct HttpServerConfig {
    port: u16,
    host_url: String,
}

pub struct HttpServer {
    port: u16,
    registry: state::AppRegistry,
}

impl HttpServer {
    pub fn new(config: HttpServerConfig, pg: Postgres) -> Self {
        let host_url_service = HostUrl::new(&config.host_url);

        let registry = state::AppRegistry::from_pg_host_url(pg, host_url_service);
        Self {
            port: config.port,
            registry,
        }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        use std::net::Ipv4Addr;
        use tokio::net::TcpListener;
        use tower_http::trace::TraceLayer;

        let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, self.port)).await?;
        let router = axum::Router::new()
            .nest("/accounts", accounts::router(self.registry.clone()))
            .nest("/.well-known", well_known::router(self.registry.clone()))
            .nest("/ap", ap::router(self.registry.clone()))
            .layer(TraceLayer::new_for_http());

        tracing::info!("Listening on {}", listener.local_addr()?);
        tracing::info!("Host URL: {}", self.registry.host_url_service().base_url());
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .inspect_err(|e| tracing::error!(error = %e,"Server error"))?;

        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

use crate::extractors::workdir::TempDirHandle;
use crate::job::check_job_state_by_id;
use crate::verifier::root;
use crate::{errors::ServerError, job::JobStore, prove, Args};

use axum::{
    middleware,
    routing::{get, post},
    serve, Router,
};
use core::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::trace;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[derive(Clone)]
pub struct AppState {
    pub job_store: JobStore,
}
pub async fn start(args: Args) -> Result<(), ServerError> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let job_store: JobStore = Arc::new(Mutex::new(Vec::new()));
    let app_state = AppState {
        job_store: job_store,
    };
    let app = Router::new()
        .route("/verify", post(root))
        .with_state(app_state.clone())
        .route("/job-status/:id", get(check_job_state_by_id))
        .with_state(app_state.clone())
        .nest("/prove", prove::router(app_state))
        .layer(middleware::from_extractor::<TempDirHandle>());
    let address: SocketAddr = format!("{}:{}", args.host, args.port)
        .parse()
        .map_err(ServerError::AddressParse)?;
    let listener = TcpListener::bind(address).await?;
    trace!("Listening on {}", address);
    serve(listener, app).await?;
    Ok(())
}

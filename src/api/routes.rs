use axum::{
    Router,
    routing::{delete, get, post, put},
};
use log::{error, info};
use ratatui::style::Stylize;
use tokio::task::JoinHandle;

use crate::api::state::AppState;
use crate::{api::handlers::*, error::AutoPilotError};

pub async fn start_api(state: AppState) -> Result<JoinHandle<()>, AutoPilotError> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/status", get(jobs_status))
        .route("/start", post(jobs_start))
        .route("/stop", post(jobs_stop))
        .route("/reload", post(jobs_reload))
        .route("/jobs", get(jobs_list))
        .route("/jobs", post(jobs_create))
        .route("/jobs/{id}", get(jobs_getbyid))
        .route("/jobs/{id}", delete(jobs_delete))
        .route("/jobs/{id}", put(jobs_update))
        .with_state(state.clone());
    const DEFAULT_PORT: &str = "7883";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PORT))
        .await
        // .inspect_err(|err| {
        //     // error!("Failed to bind API : {}", err);
        //     info!(
        //         "This is likely do to port {} not being available",
        //         DEFAULT_PORT
        //     );
        //     // std::process::exit(1);
        // })
        .map_err(|err| AutoPilotError::Api(err.to_string()))?;

    info!(
        "{}{}",
        "Api server started on 0.0.0.0:".green(),
        DEFAULT_PORT
    );
    // Spawn API server
    let api_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("API server failed");
    });
    Ok(api_handle)
}

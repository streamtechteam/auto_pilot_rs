use std::{collections::HashMap, sync::atomic::Ordering};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use crate::{
    api::state::{AppState, ReloadResponse, StatusResponse},
    job::get::get_jobs,
};
use crate::{job::JobScheme, status::JobStatusEnum};
use crate::{
    job::set::{add_job, remove_job},
    status::StateManager,
};

#[derive(Deserialize)]
pub struct StartConfig {
    verbose: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

#[derive(Serialize)]
pub struct JobResponse {
    id: String,
    name: String,
    description: String,
    loaded: bool,
    status: String,
}

pub static API_SERVICE_NAME: &str = "autopilot-api";
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: API_SERVICE_NAME.to_string(),
    })
}

pub async fn jobs_status(State(state): State<AppState>) -> Json<StatusResponse> {
    let ap = state.auto_pilot.read().await;
    Json(StatusResponse {
        running: state.started.load(Ordering::Relaxed),
        job_count: ap.jobs.len(),
    })
}

pub async fn jobs_start(
    State(state): State<AppState>,
    // Json(payload): Json<Option<StartConfig>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Prevent double-start
    if state.started.swap(true, Ordering::Relaxed) {
        return Err(StatusCode::CONFLICT);
    }

    let mut ap = state.auto_pilot.write().await;
    ap.load_jobs();
    ap.run_jobs();

    info!("Jobs started via API");
    Ok(Json(
        serde_json::json!({ "success": true, "message": "Jobs started" }),
    ))
}

pub async fn jobs_stop(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !state.started.swap(false, Ordering::Relaxed) {
        return Err(StatusCode::CONFLICT); // Already stopped
    }

    let mut ap = state.auto_pilot.write().await;
    ap.stop_jobs()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("Jobs stopped via API");
    Ok(Json(
        serde_json::json!({ "success": true, "message": "Jobs stopped" }),
    ))
}

pub async fn shutdown(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut ap = state.auto_pilot.write().await;
    if !state.started.swap(false, Ordering::Relaxed) {
        ap.shutdown()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    warn!("Autopilot shutted down via API");
    std::process::exit(0);
    // Ok(Json(
    //     serde_json::json!({ "success": true, "message": "Autopilot shutted down" }),
    // ))
}

pub async fn jobs_reload(
    State(state): State<AppState>,
) -> Result<Json<ReloadResponse>, StatusCode> {
    let mut ap = state.auto_pilot.write().await;

    ap.reload().await;

    // reload_config already calls run_jobs(), so ensure flag is set
    state.started.store(true, Ordering::Relaxed);
    Ok(Json(ReloadResponse {
        message: "Config reloaded.".to_string(),
        success: true,
    }))
}

// ============ Job CRUD Handlers ============

impl JobResponse {
    async fn from_job_and_sm(job: &crate::job::Job, sm: StateManager) -> Self {
        JobResponse {
            id: job.id.clone(),
            loaded: job.loaded,
            name: job.name.clone(),
            description: job.description.clone(),
            status: format!("{:?}", sm.get_state_by_id(job.id.clone()).await),
        }
    }
}

// impl From<&crate::job::Job> for JobResponse {
//     fn from(job: &crate::job::Job, sm: StateManager) -> Self {
//         JobResponse {
//             id: job.id.clone(),
//             loaded: job.loaded,
//             name: job.name.clone(),
//             description: job.description.clone(),
//             status: format!("{:?}", job.status),
//         }
//     }
// }
impl From<&crate::status::JobStatusStruct> for JobResponse {
    fn from(job: &crate::status::JobStatusStruct) -> Self {
        JobResponse {
            id: job.id.clone(),
            name: job.name.clone(),
            loaded: true,
            //TODO
            description: job.name.clone(),
            status: format!("{:?}", job.status),
        }
    }
}

// GET /jobs - List all jobs
// pub async fn jobs_list(
//     State(state): State<AppState>,
// ) -> Result<Json<Vec<JobResponse>>, StatusCode> {
//     let ap = state.auto_pilot.read().await;
//     // error!("api : {:p}", state.auto_pilot.as_ref());
//     // dbg!(
//     //     &ap.jobs
//     //         .iter()
//     //         .map(|job| job.status.clone())
//     //         .collect::<Vec<JobStatusEnum>>()
//     // );
//     // let jobs: Vec<JobResponse> = ap.jobs.iter().map(JobResponse::from).collect();
//     let jobs: Vec<JobResponse> = get_status_log()
//         .statuses
//         .iter()
//         .map(JobResponse::from)
//         .collect();
//     // let jobs: Vec<JobResponse> = get_jobs(true).iter().map(JobResponse::from).collect();
//     // let live_jobs =
//     Ok(Json(jobs))
// }

pub async fn jobs_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<JobResponse>>, StatusCode> {
    let ap = state.auto_pilot.read().await;

    // ۱. تبدیل live_jobs به HashMap با کلیدِ id
    // این‌ها اولویت بالاتری دارن (نسخه‌های زنده)
    let live_map: HashMap<String, JobResponse> = ap
        .status_manager
        .get_status_log()
        .await
        .statuses
        .iter()
        .map(|j| {
            let resp = JobResponse::from(j);
            (resp.id.clone(), resp)
        })
        .collect();

    // ۲. تبدیل jobs (مثلاً دیتابیس) به HashMap
    let mut db_map = HashMap::new();
    for j in get_jobs(true) {
        let resp = JobResponse::from_job_and_sm(&j, ap.status_manager.clone()).await;
        db_map.insert(resp.id.clone(), resp);
    }

    // ۳. شروع با لیست دیتابیس
    let mut merged = db_map;

    for (id, live_job) in live_map {
        merged.insert(id, live_job);
    }

    // ۵. تبدیل نهایی HashMap به Vec برای ارسال به کاربر
    let result: Vec<JobResponse> = merged.into_values().collect();

    Ok(Json(result))
}

/// POST /jobs - Create a new job
pub async fn jobs_create(
    Json(payload): Json<JobScheme>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match add_job(
        payload.name.clone(),
        payload.description.clone(),
        payload.when.clone(),
        payload.check_interval.clone(),
        payload.conditions.clone(),
        payload.tasks.clone(),
    ) {
        Ok(path) => {
            info!("Created job via API: {:?}", path);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Job created",
                "path": path.to_string_lossy()
            })))
        }
        Err(e) => {
            info!("Failed to create job: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// GET /jobs/{id} - Get job by ID
pub async fn jobs_getbyid(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<JobResponse>, StatusCode> {
    let ap = state.auto_pilot.read().await;
    if let Some(job) = ap.jobs.iter().find(|j| j.id == id) {
        let resp = JobResponse::from_job_and_sm(job, ap.status_manager.clone()).await;
        Ok(Json(resp))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

/// DELETE /jobs/{id} - Delete job by ID
pub async fn jobs_delete(Path(id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    match remove_job(Some(id), None) {
        Ok(()) => {
            info!("Deleted job via API");
            Ok(Json(
                serde_json::json!({ "success": true, "message": "Job deleted" }),
            ))
        }
        Err(e) => {
            info!("Failed to delete job: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// PUT /jobs/{id} - Update job by ID
pub async fn jobs_update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<JobScheme>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // First delete the old job
    if let Err(_) = remove_job(Some(id.clone()), None) {
        return Err(StatusCode::NOT_FOUND);
    }

    // Then create the new one
    match add_job(
        Some(format!("{}_updated", id)),
        payload.description.clone(),
        payload.when.clone(),
        payload.check_interval.clone(),
        payload.conditions.clone(),
        payload.tasks.clone(),
    ) {
        Ok(path) => {
            info!("Updated job via API: {:?}", path);
            Ok(Json(serde_json::json!({
                "success": true,
                "message": "Job updated",
                "path": path.to_string_lossy()
            })))
        }
        Err(e) => {
            info!("Failed to update job: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

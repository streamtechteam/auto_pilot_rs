use std::{collections::HashMap, sync::atomic::Ordering};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::{api::state::AppState, job::get::get_jobs};
use crate::{job::JobScheme, status::JobStatusEnum};
use crate::{
    job::set::{add_job, remove_job},
    status::get::get_status_log,
};

#[derive(Serialize)]
pub struct JobsStatus {
    running: bool,
    job_count: usize,
}

#[derive(Deserialize)]
pub struct StartConfig {
    verbose: Option<bool>,
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "autopilot-api" }))
}

pub async fn jobs_status(State(state): State<AppState>) -> Json<JobsStatus> {
    let ap = state.auto_pilot.read().await;
    Json(JobsStatus {
        running: state.started.load(Ordering::Relaxed),
        job_count: ap.jobs.len(),
    })
}

pub async fn jobs_start(
    State(state): State<AppState>,
    Json(payload): Json<Option<StartConfig>>,
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

pub async fn jobs_reload(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut ap = state.auto_pilot.write().await;
    ap.reload().await;
    // reload_config already calls run_jobs(), so ensure flag is set
    state.started.store(true, Ordering::Relaxed);

    Ok(Json(
        serde_json::json!({ "success": true, "message": "Config reloaded" }),
    ))
}

// ============ Job CRUD Handlers ============

#[derive(Serialize)]
pub struct JobResponse {
    id: String,
    name: String,
    description: String,
    status: String,
}

impl From<&crate::job::Job> for JobResponse {
    fn from(job: &crate::job::Job) -> Self {
        JobResponse {
            id: job.id.clone(),
            name: job.name.clone(),
            description: job.description.clone(),
            status: format!("{:?}", job.status),
        }
    }
}
impl From<&crate::status::JobStatusStruct> for JobResponse {
    fn from(job: &crate::status::JobStatusStruct) -> Self {
        JobResponse {
            id: job.id.clone(),
            name: job.name.clone(),
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
    let live_map: HashMap<String, JobResponse> = get_status_log()
        .statuses
        .iter()
        .map(|j| {
            let resp = JobResponse::from(j);
            (resp.id.clone(), resp)
        })
        .collect();

    // ۲. تبدیل jobs (مثلاً دیتابیس) به HashMap
    let db_map: HashMap<String, JobResponse> = get_jobs(true)
        .iter()
        .map(|j| {
            let resp = JobResponse::from(j);
            (resp.id.clone(), resp)
        })
        .collect();

    // ۳. شروع با لیست دیتابیس
    let mut merged = db_map;

    // ۴. آپدیت کردن با live_jobs
    // اگر id تکراری باشه، مقدار live_map جایگزین میشه (Overwrite)
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
    ap.jobs
        .iter()
        .find(|j| j.id == id)
        .map(|job| Ok(Json(JobResponse::from(job))))
        .unwrap_or(Err(StatusCode::NOT_FOUND))
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

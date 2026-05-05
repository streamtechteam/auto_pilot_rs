use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Serialize;
use std::sync::{Arc, atomic::AtomicBool};
use tokio::sync::RwLock;

use crate::autopilot::AutoPilot;

// Simple response types
#[derive(Serialize)]
pub struct StatusResponse {
    running: bool,
    job_count: usize,
}

#[derive(Serialize)]
pub struct ReloadResponse {
    success: bool,
    message: String,
}

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub auto_pilot: Arc<RwLock<AutoPilot>>,
    pub started: Arc<AtomicBool>,
}

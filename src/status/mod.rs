use serde::{Deserialize, Serialize};

pub mod get;
pub mod set;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StatusLog {
    pub time: String,
    pub statuses: Vec<JobStatusStruct>,
}

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::api::state;

#[derive(Clone)]
pub struct StateManager {
    inner: Arc<RwLock<StatusLog>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(StatusLog {
                time: chrono::Local::now().to_string(),
                statuses: Vec::new(),
            })),
        }
    }

    pub async fn set(&self, id: String, name: Option<String>, status: JobStatusEnum) {
        let mut log = self.inner.write().await;
        if let Some(entry) = log.statuses.iter_mut().find(|s| s.id == id) {
            entry.status = status;
        } else {
            // Optionally insert a new entry
            log.statuses.push(JobStatusStruct {
                id,
                name: name.unwrap_or(String::new()), // you may want to pass name as well
                status,
            });
        }
    }

    pub async fn get_all(&self) -> Vec<JobStatusStruct> {
        self.inner.read().await.statuses.clone()
    }
    pub async fn get_status_log(&self) -> StatusLog {
        let inner = self.inner.read().await;
        StatusLog {
            statuses: inner.statuses.clone(),
            time: inner.time.clone(),
        }
    }
    pub async fn get_state_by_id(&self, id: String) -> Option<JobStatusEnum> {
        match self.get_state_struct_by_id(id).await {
            Some(state_struct) => return Some(state_struct.status),
            None => return None,
        }
    }

    pub async fn get_state_struct_by_id(&self, id: String) -> Option<JobStatusStruct> {
        self.get_all()
            .await
            .iter()
            .find(|state| state.id == id)
            .map(|state| state.clone())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JobStatusStruct {
    pub id: String,
    pub name: String,
    pub status: JobStatusEnum,
}

impl JobStatusStruct {
    pub fn new(id: String, name: String, status: JobStatusEnum) -> Self {
        JobStatusStruct { id, name, status }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum JobStatusEnum {
    /// Job is queued but not yet started
    Pending,
    /// Job is actively executing
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed during execution
    Failed,
    /// Job was intentionally stopped
    Cancelled,
    /// Job is waiting for dependencies or conditions
    Waiting,
    /// Job didnt run due to conditions not being met
    Unsatisfied,
    /// Status cannot be determined (default state)
    Unknown,
    /// Job is scheduled but not yet started
    Scheduled,
}

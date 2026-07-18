use std::path::PathBuf;

use crate::fs::CONFIG_PATH;

pub fn get_autopilot_path() -> String {
    CONFIG_PATH
        .get()
        .expect("CONFIG_PATH Static is not initiated")
        .clone()
}

pub fn get_logs_path() -> String {
    get_autopilot_path() + "/logs"
}

pub fn get_config_path() -> String {
    get_autopilot_path() + "/autopilot.jsonc"
}

pub fn get_jobs_path() -> String {
    get_autopilot_path() + "/jobs"
}

pub fn get_jobs_dir() -> Result<PathBuf, crate::error::AutoPilotError> {
    let jobs_path = get_jobs_path();
    Ok(PathBuf::from(jobs_path))
}

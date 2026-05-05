use std::{collections::HashMap, path::PathBuf};

use serde_json::de;

use crate::{
    conditions::ConditionScheme,
    error::AutoPilotError,
    fs::{self, get_jobs_dir},
    job::{
        JobScheme,
        get::{get_job, get_jobs_paths},
    },
    task::TaskScheme,
    time::When,
};

pub fn add_job(
    name: Option<String>,
    description: Option<String>,
    when: Option<When>,
    check_interval: Option<String>,
    conditions: Vec<ConditionScheme>,
    tasks: Vec<TaskScheme>,
) -> Result<PathBuf, AutoPilotError> {
    // Generate unique ID
    let id = format!("job_{}", chrono::Utc::now().timestamp());

    let job = JobScheme {
        id: id.clone(),
        name: name.clone(),
        description,
        when,
        check_interval,
        conditions,
        tasks,
    };

    // Write to file
    let jobs_dir = get_jobs_dir()?;
    let job_file_path = jobs_dir.join(format!(
        "{}.jsonc",
        if name.is_some() {
            format!(
                "{}_{}",
                id,
                name.expect("there is something wrong with world if you are seeing this")
            )
        } else {
            id
        }
    ));

    fs::set_jobs_path().map_err(|e| {
        AutoPilotError::InvalidJob(format!("Failed to create jobs directory: {}", e))
    })?;

    std::fs::write(
        &job_file_path,
        serde_json::to_string_pretty(&job).expect("couldnt convert job to string"),
    )
    .map_err(|e| AutoPilotError::InvalidJob(format!("Failed to write job file: {}", e)))?;

    Ok(job_file_path)
}

pub fn remove_job(id: Option<String>, file_name: Option<String>) -> Result<(), AutoPilotError> {
    if id.is_none() && file_name.is_none() {
        return Err(AutoPilotError::InvalidJob(
            "No job ID or file name provided".to_string(),
        ));
    }

    if file_name.is_some() {
        // let jobs_path = get_jobs_paths();
        std::fs::remove_file(
            if file_name
                .clone()
                .expect("you should not see this")
                .ends_with(".jsonc")
            {
                file_name
                    .clone()
                    .expect("your device logic is wrong if you are seeing this")
            } else {
                format!(
                    "{}.jsonc",
                    file_name
                        .clone()
                        .expect("something is wrong with you or your device")
                )
            },
        )
        .map_err(|e| AutoPilotError::InvalidJob(format!("Failed to remove job file: {}", e)))?;
    }
    if id.is_some() && file_name.is_none() {
        let mut deleted = false;
        let jobs_path = get_jobs_paths();
        let _path_id_hashmap: HashMap<PathBuf, String> = HashMap::new();
        for path in jobs_path {
            match get_job(path.clone()) {
                Ok(value) => {
                    if value.id == id.clone().expect("you shouldnt see this") {
                        std::fs::remove_file(path).map_err(|e| {
                            AutoPilotError::InvalidJob(format!("Failed to remove job file: {}", e))
                        })?;
                        deleted = true;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        if deleted == false {
            return Err(AutoPilotError::Job(
                "Job associated with provided job id not found".to_string(),
            ));
        }
    }
    Ok(())
}

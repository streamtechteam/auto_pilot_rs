use std::{env, fs};

use log::info;

use crate::{
    error::{AutoPilotError, Result},
    fs::{CONFIG_PATH, get_autopilot_path, get_config_path, get_jobs_path, get_logs_path},
};

pub fn set_all_paths(quiet: bool) -> Result<()> {
    set_autopilot_path(None)?;
    set_logs_path()?;
    set_config_path(quiet)?;
    set_jobs_path()?;
    Ok(())
}

pub fn set_autopilot_path(config_path: Option<String>) -> Result<()> {
    let home_path = env::home_dir().expect("couldnt get home directory");
    if CONFIG_PATH.get().is_none() {
        if config_path.is_some() {
            CONFIG_PATH
                .set(config_path.unwrap())
                .expect("couldnt set CONFIG_PATH");
        } else if cfg!(target_os = "windows") {
            CONFIG_PATH
                .set(format!(
                    "{}\\AppData\\Roaming\\auto-pilot",
                    home_path.display()
                ))
                .expect("couldnt set CONFIG_PATH");
        } else {
            CONFIG_PATH
                .set(format!("{}/.config/auto-pilot", home_path.display()))
                .expect("couldnt set CONFIG_PATH");
        };
    }

    fs::create_dir_all(get_autopilot_path()).map_err(|e| {
        AutoPilotError::DirectoryInit(format!("Failed to create auto_pilot directory: {}", e))
    })
}

pub fn set_logs_path() -> Result<()> {
    let logs_path: String = get_logs_path();
    fs::create_dir_all(&logs_path).map_err(|e| {
        AutoPilotError::DirectoryInit(format!("Failed to create logs directory: {}", e))
    })
}

pub fn set_config_path(quiet: bool) -> Result<()> {
    let conf_path: String = get_config_path();
    let exists = fs::metadata(&conf_path).is_ok();
    if exists {
        if !quiet {
            info!("Config file already exist")
        }
    } else {
        fs::write(&conf_path, "{}").map_err(|e| {
            AutoPilotError::DirectoryInit(format!("Failed to create configuration file: {}", e))
        })?;
        if !quiet {
            info!("Config file created")
        }
    }
    Ok(())
}

//     Ok(())
// }

pub fn set_jobs_path() -> Result<()> {
    let jobs_path: String = get_jobs_path();
    fs::create_dir_all(&jobs_path).map_err(|e| {
        AutoPilotError::DirectoryInit(format!("Failed to create jobs directory: {}", e))
    })
}

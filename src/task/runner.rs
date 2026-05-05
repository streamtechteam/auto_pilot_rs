use log::error;

use crate::error::AutoPilotError;

pub fn sync_run(command: String) -> Result<(), AutoPilotError> {
    if let Err(e) = duct_sh::sh_dangerous(&command).run() {
        // error!("Failed to run task '{}': {}", command, e);
        return Err(AutoPilotError::JobExecution(format!(
            "Failed to run task '{}': {}",
            command, e
        )));
    }
    Ok(())
}

pub async fn async_run(command: String) -> Result<(), AutoPilotError> {
    sync_run(command)?;
    Ok(())
}

// use log::info;

use colored::*;
use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use crate::{
    error::AutoPilotError,
    fs::get_autopilot_path,
    job::set::remove_job,
    status::{get::get_status_log, set::set_status_initial},
};

pub fn list() {
    match list_interactive() {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn list_interactive() -> Result<(), AutoPilotError> {
    loop {
        // set_status_initial().expect("failed to reset status");
        let status_log = get_status_log();
        let statuses = status_log.statuses.clone();
        let formated_jobs: Vec<String> = status_log
            .statuses
            .iter()
            .map(|value| format!("{} - {}", value.id, value.name))
            .collect();

        if formated_jobs.is_empty() {
            println!("No jobs found");
            return Ok(());
        }
        let selected_job_index: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Job (↑↓ nav, Enter action, ESC back) [root: {}]",
                get_autopilot_path()
            ))
            .items(&formated_jobs)
            .default(0)
            .interact()
            .map_err(|err| AutoPilotError::Dialoguer(err))?;
        let selected_action = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an action:")
            .items(&["View Details", "Delete"])
            .default(0)
            .interact()
            .map_err(|err| AutoPilotError::Dialoguer(err))?;
        let selected_job = &statuses[selected_job_index];
        match selected_action {
            0 => {
                println!(
                    "\nID : {}\nName: {}\nStatus: {:?}\n",
                    selected_job.id.yellow(),
                    selected_job.name.green(),
                    selected_job.status
                );
                // println!("Viewing details for job {}", jobs[selected_job]);
            }
            1 => {
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Are you sure you want to delete this job?")
                    .interact()
                    .map_err(|err| AutoPilotError::Dialoguer(err))?;
                if confirm {
                    remove_job(Some(selected_job.id.clone()), None)?;
                    println!("Deleting job {}", selected_job.id);
                }
            }
            _ => unreachable!(),
        }
    }
    // Ok(())
}

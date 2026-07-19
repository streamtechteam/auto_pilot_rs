// use log::info;

use std::io::stdout;

use colored::*;
use crossterm::{execute, terminal};
use dialoguer::{Confirm, Select, theme::ColorfulTheme};

use crate::{
    error::AutoPilotError,
    fs::get_autopilot_path,
    job::{get::get_jobs, set::remove_job},
};

pub fn list() {
    match list_interactive() {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}

pub fn list_interactive() -> Result<(), AutoPilotError> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    loop {
        let jobs = get_jobs(true);

        let formated_jobs: Vec<String> = get_jobs(true)
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
        let selected_job = &jobs[selected_job_index];
        match selected_action {
            0 => {
                execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
                //
                println!(
                    "\nID : {}\nName: {}\n",
                    selected_job.id.yellow(),
                    selected_job.name.green(),
                );
                // println!("Viewing details for job {}", jobs[selected_job]);
            }
            1 => {
                execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Are you sure you want to delete this job?")
                    .interact()
                    .map_err(|err| AutoPilotError::Dialoguer(err))?;
                if confirm {
                    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
                    remove_job(Some(selected_job.id.clone()), None)?;
                    println!("Deleting job {}", selected_job.id);
                }
            }
            _ => unreachable!(),
        }
    }
    // Ok(())
}

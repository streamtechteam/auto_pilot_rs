use std::future::Future;

use tokio_cron_scheduler::JobScheduler;

use crate::{
    error::AutoPilotError,
    job::Job,
    status::JobStatusEnum,
    time::{When, to_cron_expression},
};

pub async fn add_job<Fut, F>(
    job: &Job,
    scheduler: &JobScheduler,
    run_job: F,
) -> Result<(), AutoPilotError>
where
    Fut: Future<Output = ()> + Send + 'static,
    F: Fn(Job) -> Fut + Send + Sync + Clone + 'static,
{
    // Validate job time
    let when = match job.when.clone() {
        Some(when) => when,
        None => {
            return Err(AutoPilotError::Time("Job time is None".to_string()));
        }
    };

    // These variables are only used during interactive creation in cli/create.rs
    // For this module, we only need to use them in this match arm
    let mut _once = true;
    let mut _cron_exp = String::new();
    let cron_job: tokio_cron_scheduler::Job;

    match &when {
        When::Once(value) => {
            let target_time = value
                .parse()
                .map_err(|err| AutoPilotError::Time(format!("Failed to parse time : {}", err)))?;
            let now = chrono::Local::now();
            if target_time <= now {
                return Err(AutoPilotError::Time("Job time is in the past".to_string()));
            }

            let duration = (target_time - now)
                .to_std()
                .map_err(|e| AutoPilotError::Time(format!("Failed to compute duration: {}", e)))?;

            let job_clone = job.clone();
            let run_job_clone = run_job.clone();

            cron_job = tokio_cron_scheduler::Job::new_one_shot_async(duration, move |_, _| {
                println!("Time {:?}", chrono::Local::now());
                let job = job_clone.clone();
                let run_job = run_job_clone.clone();

                Box::pin(async move {
                    run_job(job);
                })
            })?;
            // Add job to scheduler
            scheduler.add(cron_job).await?;
        }
        When::Cron(expression) => {
            let job_clone = job.clone();
            let run_job_clone = run_job.clone();

            cron_job = tokio_cron_scheduler::Job::new(expression.as_str(), move |_, _| {
                let job = job_clone.clone();
                run_job_clone(job);
            })?;
            scheduler.add(cron_job).await?;
            _once = false;
        }
        _ => {
            let job_clone = job.clone();
            let run_job_clone = run_job.clone();
            let cron_expression = to_cron_expression(when).unwrap();

            cron_job = tokio_cron_scheduler::Job::new(cron_expression.as_str(), move |_, _| {
                let job = job_clone.clone();
                run_job_clone(job);
            })?;
            scheduler.add(cron_job).await?;
            _once = false;
        }
    }

    // The following variables are still unused, but now they are only warnings.
    // _cron_exp = to_cron_expression(when).unwrap();

    Ok(())
}

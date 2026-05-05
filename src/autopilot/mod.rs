use colored::Colorize;
use futures::future::join_all;
use log::{error, info, warn};
use tokio::task::JoinHandle;
use tokio_cron_scheduler::JobScheduler;

use crate::{
    cli::status::check_if_running,
    error::AutoPilotError,
    job::{Job, get::get_jobs},
    logging::init_logging,
    status::set::set_status_initial,
    time::init::init_time_check,
};

// #[derive(Clone)]
pub struct AutoPilot {
    pub started: bool,
    pub scheduler: JobScheduler,
    pub jobs: Vec<Job>,
    pub jobs_handles: Vec<JoinHandle<()>>,
}

impl AutoPilot {
    pub async fn new() -> Self {
        Self {
            started: false,
            scheduler: init_time_check().await.expect("failed to init cron"),
            jobs: Vec::new(),
            jobs_handles: Vec::new(),
        }
    }

    pub fn init(&mut self, verbose: bool) -> Result<(), AutoPilotError> {
        Self::prepare_logging(verbose);
        if Self::check_instance() {
            return Err(AutoPilotError::Autopilot(
                "Instance already running".to_string(),
            ));
        }
        Self::init_status().expect("failed to init status");
        self.load_jobs();
        Ok(())
    }
    pub async fn reload(&mut self) {
        self.stop_jobs().await.expect("failed to stop jobs");
        info!("{}", "Reloading Autopilot...".yellow());
        Self::init_status().expect("failed to init status");
        self.load_jobs();

        self.start(false);

        info!("{}", "Autopilot reloaded successfully!".green())
    }
    pub fn start(&mut self, verbose: bool) {
        // Self::prepare_logging(verbose);
        if Self::check_instance() {
            return;
        }
        // Self::init_status().expect("failed to init status");
        // self.load_jobs();
        self.jobs_handles = self.run_jobs();
        // dbg!(&self.jobs_handles);
        info!("{}", "Autopilot served!".green());
    }
    pub fn check_instance() -> bool {
        check_if_running()
    }
    pub fn init_status() -> Result<(), AutoPilotError> {
        if let Err(e) = set_status_initial() {
            error!("Failed to initialize status: {}", e);
            return Err(AutoPilotError::State(e));
        }
        Ok(())
    }
    pub fn run_jobs(&mut self) -> Vec<JoinHandle<()>> {
        let mut handles = vec![];
        for mut job in self.jobs.clone() {
            let scheduler = self.scheduler.clone();
            handles.push(tokio::task::spawn(async move {
                job.run(&scheduler, false).await;
            }))
        }

        handles
    }
    pub fn load_jobs(&mut self) {
        self.jobs = get_jobs(false);
    }

    /// Stop all jobs (graceful shutdown of scheduler)
    pub async fn stop_jobs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        warn!("Stopping jobs...");
        // dbg!(&self.jobs_handles);
        self.jobs_handles.iter().for_each(|handle| {
            // info!("Aborting job handle");
            handle.abort();
        });
        self.scheduler.shutdown().await?;
        self.jobs = vec![];
        self.jobs_handles = vec![];

        // self.load_jobs();
        // Optionally clear jobs vector or reset state
        Ok(())
    }
    // fn add_job(&mut self, job: Job) {
    //     self.jobs.push(job);
    // }
    // fn remove_job(&mut self, job: Job) {
    //     // self.jobs.remove(self.jobs.iter().index)
    // }
    pub fn prepare_logging(verbose: bool) {
        init_logging(verbose);
    }
}

use std::{sync::Arc, thread::sleep, time::Duration};

use colored::Colorize;
use futures::future::join_all;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle};
use tokio_cron_scheduler::JobScheduler;

use crate::{
    conditions::{Condition, ConditionScheme},
    error::AutoPilotError,
    status::{JobStatusEnum, StateManager},
    task::{self, Task, TaskScheme},
    time::{When, add::add_job},
};

pub mod get;
pub mod set;

#[derive(Clone)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub description: String,
    pub when: Option<When>,
    pub check_interval: Option<String>,
    pub conditions: Vec<Box<dyn Condition>>,
    pub tasks: Vec<task::Task>,
    pub loaded: bool,
}

impl Job {
    pub fn new(
        id: String,
        name: String,
        description: String,
        check_interval: Option<String>,
        when: Option<When>,
        conditions: Vec<Box<dyn Condition>>,
        tasks: Vec<task::Task>,
    ) -> Self {
        Job {
            id,
            name,
            description,
            when,
            check_interval,
            conditions,
            tasks,
            loaded: false,
        }
    }

    pub fn from_scheme(scheme: JobScheme) -> Self {
        let conditions: Vec<Box<dyn Condition>> = scheme
            .conditions
            .into_iter()
            .map(|condition_scheme| condition_scheme.to_condition())
            .collect();

        let tasks: Vec<task::Task> = scheme
            .tasks
            .into_iter()
            .map(|task_scheme| task::Task::new(task_scheme.command))
            .collect();

        let when = scheme.when;
        Job {
            id: scheme.id.clone(),
            name: scheme.name.unwrap_or(format!("job_{}", scheme.id)),
            description: scheme.description.unwrap_or(" ".to_string()),
            when,
            check_interval: scheme.check_interval,
            conditions,
            tasks,
            loaded: false,
        }
    }

    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        let scheme: JobScheme = serde_json::from_str(json_str)?;
        Ok(Job::from_scheme(scheme))
    }

    pub fn add_condition(&mut self, condition: Box<dyn Condition>) {
        self.conditions.push(condition);
    }
    pub async fn run(
        &mut self,
        state_manager: StateManager,
        scheduler: &JobScheduler,
        quiet: bool,
    ) {
        if !quiet {
            info!("{} : {}", "Running job".yellow(), self.name);
        }
        // self.status = JobStatusEnum::Running;
        // if let Err(e) = set_state_item(self.id.clone(), JobStatusEnum::Running) {
        //     error!("Failed to set state item: {}", e);
        // }

        state_manager
            .set(self.id.clone(), None, JobStatusEnum::Running)
            .await;

        // if self.when.is_none() {
        //     loop {
        //         let mut result = true;
        //         for condition in &self.conditions {
        //             let condition_result = condition.check();
        //             result = result && condition_result;
        //         }
        //         if result {
        //             run_tasks(self.tasks.clone()).await;
        //             self.status = JobStatusEnum::Completed;
        //             // dbg!(self.status.clone());
        //             if let Err(e) = set_state_item(self.id.clone(), JobStatusEnum::Completed) {
        //                 error!("Failed to set state item: {}", e);
        //             }
        //             if !quiet {
        //                 info!("{} : {}", "Job Completed".green(), self.name);
        //             }
        //             break;
        //         } else if !result && self.check_interval.is_some() {
        //             let interval_ms = match self
        //                 .check_interval
        //                 .as_ref()
        //                 .expect("WTF you should NOT be seeing this")
        //                 .parse::<u64>()
        //             {
        //                 Ok(ms) => ms,
        //                 Err(_) => {
        //                     error!("check_interval value is not valid, using 1000ms as default");
        //                     1000
        //                 }
        //             };
        //             // sleep(Duration::from_millis(interval_ms));
        //             tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        //             // loop {
        //             //     print!("FUCK");
        //             // }
        //             continue;
        //         }
        //         self.status = JobStatusEnum::Unsatisfied;
        //         if let Err(e) = set_state_item(self.id.clone(), JobStatusEnum::Unsatisfied) {
        //             error!("Failed to set state item: {}", e);
        //         }
        //         if !quiet {
        //             info!("{} : {}", "Job Unsatisfied".yellow(), self.name);
        //         }
        //         break;
        //     }
        if self.when.is_none() {
            loop {
                let mut result = true;
                for condition in self.conditions.clone() {
                    // Clone the whole vector per iteration (if conditions are Clone)
                    // Run the heavy check on the blocking pool
                    let condition_result = tokio::task::spawn_blocking(move || condition.check())
                        .await
                        .expect("condition check panicked");
                    result = result && condition_result;
                }
                if result {
                    run_tasks(self.tasks.clone()).await;
                    // self.status = JobStatusEnum::Completed;
                    // dbg!(self.status.clone());
                    // if let Err(e) =
                    //     set_status_item(self.id.clone(), None, JobStatusEnum::Completed).await
                    // {
                    //     error!("Failed to set state item: {}", e);
                    // }
                    state_manager
                        .set(self.id.clone(), None, JobStatusEnum::Completed)
                        .await;
                    if !quiet {
                        info!("{} : {}", "Job Completed".green(), self.name);
                    }
                    break;
                    // ... set status, break
                } else if self.check_interval.is_some() {
                    // sleep, then continue the loop
                    let interval_ms = match self
                        .check_interval
                        .as_ref()
                        .expect("WTF you should NOT be seeing this")
                        .parse::<u64>()
                    {
                        Ok(ms) => ms,
                        Err(_) => {
                            error!("check_interval value is not valid, using 1000ms as default");
                            1000
                        }
                    };
                    // sleep(Duration::from_millis(interval_ms));
                    // tokio::time::sleep(Duration::from_millis(interval_ms)).await;
                    // loop {
                    //     print!("FUCK");
                    // }
                    // continue;
                    tokio::time::sleep(Duration::from_millis(interval_ms)).await;
                    continue;
                } else {
                    // self.status = JobStatusEnum::Unsatisfied;
                    // if let Err(e) = set_state_item(self.id.clone(), JobStatusEnum::Unsatisfied) {
                    //     error!("Failed to set state item: {}", e);
                    // }
                    state_manager
                        .set(self.id.clone(), None, JobStatusEnum::Unsatisfied)
                        .await;
                    if !quiet {
                        info!("{} : {}", "Job Unsatisfied".yellow(), self.name);
                    }
                    break;
                }
            }
        } else if self.when.is_some() {
            // self.status = JobStatusEnum::Scheduled;
            state_manager
                .set(self.id.clone(), None, JobStatusEnum::Scheduled)
                .await;
            let _result = add_job(self, scheduler, run_job, state_manager.clone()).await;
            if let Err(error) = _result {
                error!("{} : {}", self.name, error);
            }
        };
    }
}

// pub async fn run_job(job: Job) {
//     if let Err(e) = set_state_item(job.id.clone(), JobStatusEnum::Running) {
//         error!("Failed to set state item: {}", e);
//     }
//     let mut result = true;
//     for condition in &job.conditions {
//         let condition_result = condition.check();
//         result = result && condition_result;
//     }
//     if result {
//         run_tasks(job.tasks.clone()).await;
//         if let Err(e) = set_state_item(job.id, JobStatusEnum::Completed) {
//             error!("Failed to set state item: {}", e);
//         }
//     } else if !result {
//         if let Err(e) = set_state_item(job.id.clone(), JobStatusEnum::Unsatisfied) {
//             error!("Failed to set state item: {}", e);
//         }
//     }
// }

pub async fn run_job(job: Job, state_manager: StateManager) {
    // Set status to Running
    // if let Err(e) = set_state_item(job.id.clone(), JobStatusEnum::Running) {
    //     error!("Failed to set state item: {}", e);
    // }
    println!("nigga 0");
    state_manager
        .set(job.id.clone(), None, JobStatusEnum::Running)
        .await;
    println!("nigga 1");
    // Offload every condition.check() to the blocking pool, run in parallel
    let mut condition_futures = Vec::new();
    for condition in job.conditions {
        println!("nigga 2");

        let fut = tokio::task::spawn_blocking(move || condition.check());
        condition_futures.push(fut);
    }
    println!("nigga 3");

    let results = futures::future::join_all(condition_futures).await;
    // Check if all conditions passed
    let all_ok = results.into_iter().all(|res| res.unwrap_or(false));

    if all_ok {
        println!("nigga 4");
        run_tasks(job.tasks.clone()).await;
        // if let Err(e) = set_state_item(job.id, JobStatusEnum::Completed) {
        //     error!("Failed to set state item: {}", e);
        // }
        state_manager
            .set(job.id.clone(), None, JobStatusEnum::Completed)
            .await;
    } else {
        println!("nigga 5");
        // if let Err(e) = set_state_item(job.id, JobStatusEnum::Unsatisfied) {
        //     error!("Failed to set state item: {}", e);
        // }
        state_manager
            .set(job.id.clone(), None, JobStatusEnum::Unsatisfied)
            .await;
    }
}

pub async fn run_tasks(tasks: Vec<Task>) {
    let mut handles: Vec<JoinHandle<Result<(), AutoPilotError>>> = vec![];
    for task in &tasks {
        handles.push(task.run());
    }
    join_all(handles)
        .await
        .iter()
        .for_each(|handle| match handle {
            Ok(handle) => match handle {
                Ok(_) => {}
                Err(err) => {
                    error!("{}", err)
                }
            },
            Err(err) => {
                error!("Failed to join handles : {}", err);
            }
        });
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobScheme {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub when: Option<When>,
    pub check_interval: Option<String>,
    pub conditions: Vec<ConditionScheme>,
    pub tasks: Vec<TaskScheme>,
}

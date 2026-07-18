use std::fs;

use crate::{
    job::get::get_jobs,
    status::{JobStatusEnum, JobStatusStruct, StatusLog},
};

// pub fn set_state_item(id: String, status: JobStatusEnum) -> Result<(), String> {
//     println!("changing state : {} {:?}", id, status);
//     let state_path = get_status_path();
//     let mut status_log = get_status_log().clone();
//     let mut statuses = status_log
//         .statuses
//         .clone()
//         .into_iter()
//         .collect::<Vec<JobStatusStruct>>();
//     let index = statuses.iter().position(|item| item.id == id);

//     match index {
//         Some(index) => {
//             statuses[index].status = status;
//         }
//         None => {
//             println!("none");
//         }
//     }
//     status_log.statuses = statuses;
//     let json = serde_json::to_string_pretty(&status_log).map_err(|e| e.to_string())?;
//     // println!("json2 {}", json);
//     fs::write(&state_path, json).map_err(|e| e.to_string())?;
//     Ok(())
// }

// pub fn set_status_initial() -> Result<(), String> {
//     let state_path = get_status_path();
//     let mut status_log: StatusLog = StatusLog {
//         time: chrono::Local::now().to_string(),
//         statuses: Vec::new(),
//     };

//     for job in get_jobs(true) {
//         status_log.statuses.push(JobStatusStruct {
//             id: job.id,
//             name: job.name,
//             status: JobStatusEnum::Unknown,
//         });
//     }
//     let json = serde_json::to_string_pretty(&status_log).map_err(|e| e.to_string())?;
//     fs::write(&state_path, json).map_err(|e| e.to_string())?;

//     Ok(())
// }

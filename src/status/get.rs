use chrono::Local;
use log::error;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::{fs, sync::Mutex};

use std::sync::OnceLock;

// use serde_json::value;

use crate::{
    status::{JobStatusEnum, StatusLog},
    utilities,
};

// struct  {
//     value: u32,
// // }

// pub fn get_status_log_mem() -> &'static Mutex<StatusLog> {
//     static INSTANCE: OnceLock<Mutex<StatusLog>> = OnceLock::new();
//     INSTANCE.get_or_init(|| Mutex::new(get_status_log_from_disk()))
// }

// pub fn set_status_log_mem(log: StatusLog) {
//     let mut status_log = get_status_log_mem().lock().unwrap();
//     *status_log = log;
// }

// pub fn get_status_log() -> StatusLog {
//     let state_path = get_status_path();
//     let state_string = match fs::read_to_string(state_path) {
//         Ok(content) => content,
//         Err(e) => {
//             if log::log_enabled!(log::Level::Error) {
//                 error!("Failed to read state file: {}", e);
//             } else {
//                 eprintln!("Failed to read state file: {}", e);
//             }
//             // Initialize status and retry
//             if let Err(init_e) = set_status_initial() {
//                 if log::log_enabled!(log::Level::Error) {
//                     error!("Failed to initialize state: {}", init_e);
//                 } else {
//                     eprintln!("Failed to initialize state: {}", init_e);
//                 }
//                 // Return a default empty status log
//                 return StatusLog {
//                     time: Local::now().to_rfc3339(),
//                     statuses: vec![],
//                 };
//             }
//             return get_status_log();
//         }
//     };
//     let status_log: StatusLog =
//         match serde_json::from_str(utilities::jsonc_parser::jsonc_parse(&state_string).as_str()) {
//             Ok(value) => value,
//             Err(e) => {
//                 // Use the index `i` directly instead of searching again

//                 if log::log_enabled!(log::Level::Error) {
//                     error!("Failed to parse state file: \n Error: {}", e);
//                 } else {
//                     eprintln!("Failed to parse state file: \n Error: {}", e);
//                 }
//                 // if let Err(init_e) = set_status_initial() {
//                 //     if log::log_enabled!(log::Level::Error) {
//                 //         error!("Failed to initialize state: {}", init_e);
//                 //     } else {
//                 //         eprintln!("Failed to initialize state: {}", init_e);
//                 //     }
//                 //     // Return a default empty status log
//                 //     return StatusLog {
//                 //         time: Local::now().to_rfc3339(),
//                 //         statuses: vec![],
//                 //     };
//                 // }
//                 // vec![]
//                 get_status_log()
//             }
//         };
//     status_log
// }

// // pub fn get_status_log() -> StatusLog {
// //     get_status_log_mem()
// // }

// pub fn update_status_log_in_mem() {}

// pub fn get_job_status(id: String) -> JobStatusEnum {
//     let status_log = get_status_log();
//     // println!("{:?}", status_log);
//     status_log
//         .statuses
//         .iter()
//         .find(|job| job.id == id)
//         .map_or(JobStatusEnum::Unknown, |job| job.status.clone())
// }

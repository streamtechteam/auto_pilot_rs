use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoPilotError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(String),

    #[error("Job error: {0}")]
    Job(String),

    #[error("Job execution error: {0}")]
    JobExecution(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("State management error: {0}")]
    State(String),

    #[error("Directory initialization error: {0}")]
    DirectoryInit(String),

    #[error("Command execution error: {0}")]
    Command(String),

    #[error("Invalid job definition: {0}")]
    InvalidJob(String),

    #[error("Dialoguer error: {0}")]
    Dialoguer(#[from] dialoguer::Error),

    #[error("Signal handling error: {0}")]
    Signal(String),

    #[error("Cron scheduler error: {0}")]
    Cron(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error("Condition conversion error: {0}")]
    Condition(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Time error: {0}")]
    Time(String),

    #[error("Autopilot error: {0}")]
    Autopilot(String),

    #[error("Api Error: {0}")]
    Api(String),
}

// Type alias for convenience
pub type Result<T> = std::result::Result<T, AutoPilotError>;

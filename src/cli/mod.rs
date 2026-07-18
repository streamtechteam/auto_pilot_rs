use clap::{Parser, Subcommand, crate_version};

use crate::{
    cli::{
        create::create, gui::gui, list::list, remove::remove, serve::serve, status::status,
        stop::stop,
    },
    fs::{set_all_paths, set_autopilot_path},
};

pub mod create;
pub mod gui;
pub mod list;
pub mod remove;
pub mod serve;
pub mod status;
pub mod stop;

#[derive(Parser)]
#[command(name = "AutoPilot-rs")]
#[command(about = "A cross platform automation tool", version = crate_version!())]
struct Cli {
    #[arg(long)]
    config_path: Option<String>,

    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Subcommand
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Serve AutoPilot-rs
    Serve {
        #[arg(long, default_value_t = false)]
        api: bool,
    },
    /// Stop AutoPilot-rs
    Stop,
    /// Create a new Job
    Create,
    /// Remove a Job
    Remove,
    /// List Jobs
    List,
    /// Status of AutoPilot-rs
    Status,
    /// Run GUI
    Gui,
}

pub async fn handle_cli() {
    let cli = Cli::parse();
    handle_dir(cli.config_path.clone());
    match &cli.command {
        Some(Commands::Serve { api }) => {
            serve(cli.verbose, *api).await;
        }
        Some(Commands::Create) => {
            create();
        }
        Some(Commands::Remove) => {
            remove();
        }

        Some(Commands::Stop) => {
            stop(false);
        }
        Some(Commands::List) => {
            list();
        }
        Some(Commands::Status) => {
            status();
        }
        Some(Commands::Gui) => {
            gui().expect("failed to launch gui due to :");
        }

        None => {
            println!("No commands specified.");
            let mut cmd = <Cli as clap::CommandFactory>::command();
            cmd.print_help().unwrap();
            println!("Launching GUI...");
            gui().expect("failed to launch gui");
            return;
        }
    }
}

fn handle_dir(config_path: Option<String>) {
    // println!("cli config path : {}", config_path.clone().unwrap());

    set_autopilot_path(config_path.clone()).expect("Failed to setup dirs");
    if let Err(e) = set_all_paths(false) {
        eprintln!("Failed to set up directories: {}", e);
        std::process::exit(1);
    }

    // println!("CONFIG_PATH {}", CONFIG_PATH.get().unwrap());
}

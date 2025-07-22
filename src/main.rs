use clap::Parser;
use picode::cli::{CliArgs, Command};
use picode::config::Config;
use picode::error::Result;
use picode::logging::configure_logger;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging and configuration
    configure_logger();
    info!("Starting PiCode v{}", env!("CARGO_PKG_VERSION"));

    // Parse command line arguments
    let args = CliArgs::parse();
    
    // Load configuration
    let config = Config::try_from(&args).await?;
    
    // Execute command based on CLI input
    match args.command {
        Some(Command::Interactive(opts)) => {
            info!("Starting interactive mode");
            picode::interactive::run(opts, config).await
        },
        Some(Command::Execute { command, provider }) => {
            info!("Executing command: {:?} with provider: {:?}", command, provider);
            picode::execute::run_command(command, provider, config).await
        },
        Some(Command::Config(config_cmd)) => {
            info!("Managing configuration");
            picode::config::handle_command(config_cmd).await
        },
        Some(Command::Hooks(hooks_cmd)) => {
            info!("Managing hooks");
            picode::hooks::handle_command(hooks_cmd).await
        },
        None => {
            // Default to interactive mode
            info!("No command specified, starting interactive mode");
            let default_opts = picode::interactive::InteractiveOptions::default();
            picode::interactive::run(default_opts, config).await
        },
    }
}
use clap::Parser;
use picode::cli::CliArgs;
use picode::config::Config;
use picode::error::Result;
use picode::logging::configure_logger;
use tracing::info;

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
        picode_cli::Commands::Init { path, name, template: _, force: _ } => {
            info!("Initializing workspace at: {}", path.display());
            println!("🎯 PiCode Initialization");
            if let Some(name) = name {
                println!("Creating workspace: {}", name);
            }
            println!("Location: {}", path.display());
            println!("✅ Workspace initialized successfully");
            Ok(())
        },
        picode_cli::Commands::Workspace { ai, provider, endpoint: _, session } => {
            info!("Starting workspace mode");
            let opts = picode::interactive::InteractiveOptions {
                debug: args.debug,
                layout: "default".to_string(),
                provider: provider.map(|p| format!("{:?}", p).to_lowercase()),
            };
            
            if ai {
                println!("🤖 AI assistance enabled");
            }
            if let Some(session) = session {
                println!("📝 Session: {}", session);
            }
            
            picode::interactive::run(opts, config).await
        },
        picode_cli::Commands::Execute { command, args: cmd_args, suggest: _, dry_run: _ } => {
            info!("Executing command: {:?}", command);
            let full_command = if cmd_args.is_empty() {
                command
            } else {
                format!("{} {}", command, cmd_args.join(" "))
            };
            picode::execute::run_command(full_command, None, config).await
        },
        picode_cli::Commands::Config { action } => {
            info!("Configuration management");
            println!("⚙️ Configuration: {:?}", action);
            println!("Configuration management not fully implemented yet");
            Ok(())
        },
        picode_cli::Commands::Git { action } => {
            info!("Git integration");
            println!("📝 Git action: {:?}", action);
            println!("Git integration not implemented yet");
            Ok(())
        },
        picode_cli::Commands::Llm { action } => {
            info!("LLM provider management");
            println!("🤖 LLM action: {:?}", action);
            println!("LLM management not implemented yet");
            Ok(())
        },
        picode_cli::Commands::Plugin { action } => {
            info!("Plugin management");
            println!("🔌 Plugin action: {:?}", action);
            println!("Plugin management not implemented yet");
            Ok(())
        },
        picode_cli::Commands::Dev { action } => {
            info!("Development utilities");
            println!("🛠️ Dev action: {:?}", action);
            println!("Development utilities not implemented yet");
            Ok(())
        },
    }
}
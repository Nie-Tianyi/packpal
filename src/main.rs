use clap::{Parser as ClapParser, Subcommand};
use std::error::Error;

mod fn_build;
mod fn_new;
#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { project_name: String },
    Build { output_dir: Option<String> },
    Deploy,
    Update,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name } => fn_new::new_project(project_name),
        Commands::Build { .. } => {}
        Commands::Deploy => {}
        Commands::Update => {}
    }

    Ok(())
}

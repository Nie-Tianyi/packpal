use clap::{Parser as ClapParser, Subcommand};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
use std::error::Error;
use std::fs;
mod fn_new;
mod fn_build;
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
static POSTS_TEMPLATE: &str = include_str!("templates/posts_template.html");

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name } => {
            fn_new::new_project(project_name)
        }
        Commands::Build { .. } => {}
        Commands::Deploy => {}
        Commands::Update => {}
    }

    Ok(())
}

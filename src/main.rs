use clap::{Parser as ClapParser, Subcommand};
use std::error::Error;

mod func_build;
mod func_clean;
mod func_deploy;
mod func_new;
#[derive(ClapParser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// packpal new [name]   在当前目录下创建名字为[name]的新项目
// packpal build        生成静态站点文件，根据模板和markdown文件
// packpal deploy       将生成的静态文件部署到Github pages
// packpal update       就等于先build，再deploy
// packpal clean        清楚生成的所有文件
#[derive(Subcommand)]
enum Commands {
    New { project_name: String },
    Build { output_dir: Option<String> },
    Deploy,
    Update,
    Clean,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { project_name } => func_new::new_project(project_name),
        Commands::Build { .. } => {}
        Commands::Deploy => {}
        Commands::Update => {}
        Commands::Clean => {}
    }

    Ok(())
}

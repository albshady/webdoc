extern crate webbrowser;

use clap::{Parser, Subcommand};
use std::fs;
use std::path;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to a directory where project files are stored
    #[clap(env)]
    webdoc_projects_dir: path::PathBuf,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lists all projects
    List,

    /// Opens project's documentation in the browser
    Open {
        /// Name of the project to open a doc for
        #[clap(value_parser)]
        project: String,
    },
}

struct ProjectConfig {
    documentation_url: String,
}

impl ProjectConfig {
    fn load(project_name: &String, projects_dir: path::PathBuf) -> Self {
        let mut project_filepath = projects_dir;
        project_filepath.push(project_name);
        let documentation_url = fs::read_to_string(project_filepath).unwrap();

        Self { documentation_url }
    }

    fn browse(&self) {
        webbrowser::open(&self.documentation_url).expect("Failed to open documentation in browser");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List => list_all_projects(cli.webdoc_projects_dir),
        Command::Open { project } => open_project_doc(&project, cli.webdoc_projects_dir),
    }
}

fn list_all_projects(projects_dir: path::PathBuf) {
    let project_files = get_all_project_files(projects_dir);

    println!("Known projects:");
    for project_file in project_files {
        let project_name = project_file.file_name().unwrap();
        println!("  - {}", project_name.to_str().unwrap());
    }
}

fn get_all_project_files(projects_dir: path::PathBuf) -> Vec<path::PathBuf> {
    fs::read_dir(projects_dir)
        .expect("Error opening projects directory")
        .into_iter()
        .map(|x| x.map(|entry| entry.path()).unwrap())
        .collect()
}

fn open_project_doc(project_name: &String, projects_dir: path::PathBuf) {
    let project_config = ProjectConfig::load(project_name, projects_dir);
    project_config.browse();
}

extern crate webbrowser;

use clap::{Parser, Subcommand};
use std::fs;
use std::path;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
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
    url: String,
}

impl ProjectConfig {
    fn load(project_name: &String) -> Self {
        let mut config_filepath = get_projects_dir();
        config_filepath.push(project_name);

        let url = fs::read_to_string(config_filepath).unwrap();

        Self { url }
    }

    fn browse(&self) {
        webbrowser::open(&self.url).expect("Failed to open documentation in browser");
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::List => list_all_projects(),
        Command::Open { project } => open_project_doc(&project),
    }
}

fn list_all_projects() {
    let projects_dir = get_projects_dir();
    let project_files = get_all_project_files(projects_dir);

    println!("Known projects:");
    for project_file in project_files {
        let project_name = project_file.file_name().unwrap();
        println!("  - {}", project_name.to_str().unwrap());
    }
}

fn get_projects_dir() -> path::PathBuf {
    path::PathBuf::from("./projects/")
}

fn get_all_project_files(projects_dir: path::PathBuf) -> Vec<path::PathBuf> {
    fs::read_dir(projects_dir)
        .expect("Error opening projects directory")
        .into_iter()
        .map(|x| x.map(|entry| entry.path()).unwrap())
        .collect()
}

fn open_project_doc(project_name: &String) {
    let project_config = ProjectConfig::load(project_name);
    project_config.browse();
}

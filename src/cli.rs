use std::path::PathBuf;
use structopt::StructOpt;
use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        /// description
        #[structopt()]
        task: String
    },
    Done {
        /// position
        #[structopt()]
        position: usize
    },
    /// List all
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rusty Journal",
    about = "Journal"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

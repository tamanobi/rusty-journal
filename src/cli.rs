use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        /// description
        #[structopt()]
        task: String,
    },
    Done {
        /// position
        #[structopt()]
        position: usize,
    },
    /// List all
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Journal", about = "Journal")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

use anyhow::anyhow;
use structopt::StructOpt;
mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();
    // dbg!(&args);

    let journal_path = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;
    match action {
        Add { task } => tasks::add_task(journal_path, Task::new(task)),
        List => tasks::list_tasks(journal_path),
        Done { position } => tasks::complete_task(journal_path, position),
    }?;
    Ok(())
}

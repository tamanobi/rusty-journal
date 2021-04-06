use structopt::StructOpt;
mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();
    // dbg!(&args);

    let journal_path = journal_file.expect("Failed to find journal file");
    match action {
        Add { task } => tasks::add_task(journal_path, Task::new(task)),
        List => tasks::list_tasks(journal_path),
        Done { position } => tasks::complete_task(journal_path, position),
    }
    .expect("Failed to perform action")
}

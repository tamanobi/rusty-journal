use std::fs::OpenOptions;
use std::io::{BufReader, Result, Seek, SeekFrom};

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    // Consume the file's contents as a vetcor of tasks.
    let mut tasks: Vec<Tasks> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    // Rewind the file after reading from it.
    file.seek(SeekFrom::Start(0))?;

    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

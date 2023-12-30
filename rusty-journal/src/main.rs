use crate::cli::{Action, CommandLineArgs};
use crate::tasks::{add_task, complete_task, list_tasks, Task};
use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;

mod cli;
mod tasks;

fn main() -> anyhow::Result<()> {
    let CommandLineArgs { command, file } = CommandLineArgs::from_args();
    dbg!(&command);

    // let file = file.or(get_default_file());
    let file = file
        .or_else(get_default_file)
        .ok_or(anyhow!("Fail to find journal file"))?;
    dbg!(&file);

    match command {
        Action::Add { task } => add_task(file, Task::new(task)),
        Action::Done { index } => complete_task(file, index),
        Action::List => list_tasks(file),
    }?;

    Ok(())
}

fn get_default_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

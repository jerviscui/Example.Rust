use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::io::{Result, Seek, SeekFrom};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,

    // #[serde(with = "ts_seconds")]
    pub done_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Utc::now(),
            done_at: None,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let time = self.created_at.with_timezone(&Local);
        write!(f, "{:<50} [{}]", self.text, time)
    }
}

pub fn add_task(path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

    // let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
    //     Ok(tasks) => tasks,
    //     Err(e) if e.is_eof() => Vec::new(),
    //     Err(e) => return Err(e.into()),
    // };
    let mut tasks = get_tasks(&file)?;
    tasks.push(task);

    serde_json::to_writer(&file, &tasks)?;

    Ok(())
}

fn get_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;

    let result = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;

    Ok(result)
}

pub fn complete_task(path: PathBuf, index: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

    let mut tasks = get_tasks(&file)?;

    let index = index - 1;
    if tasks.get(index).is_none() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid task index"));
    }

    tasks.remove(index);

    file.set_len(0)?;
    serde_json::to_writer(&file, &tasks)?;

    Ok(())
}

pub fn list_tasks(path: PathBuf) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let tasks = get_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty");
        return Ok(());
    }

    for i in 0..tasks.len() {
        println!("{} {}", i +1, tasks.get(i).unwrap());
    }

    Ok(())
}

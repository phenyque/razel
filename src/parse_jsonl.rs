use std::fs::File;
use std::io::{BufRead, BufReader};

use serde::Deserialize;

use crate::{CustomCommand, Scheduler};

pub fn parse_jsonl_file(scheduler: &mut Scheduler, file_name: String) -> Result<(), anyhow::Error> {
    let file = File::open(file_name)?;
    let file_buffered = BufReader::new(file);
    for line_result in file_buffered.lines() {
        let line = line_result?;
        let line_trimmed = line.trim();
        if line_trimmed.starts_with("//") {
            continue;
        }
        let json: RazelJson = serde_json::from_str(line_trimmed)?;
        match json {
            RazelJson::CustomCommand(c) => {
                let command = Box::new(CustomCommand::new(c.executable, c.args));
                scheduler.push(command);
            }
            RazelJson::Task(t) => {
                todo!()
            }
        }
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RazelJson {
    CustomCommand(RazelCustomCommandJson),
    Task(RazelTaskJson),
}

#[derive(Deserialize)]
struct RazelCustomCommandJson {
    name: String,
    executable: String,
    args: Vec<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

#[derive(Deserialize)]
struct RazelTaskJson {
    name: String,
    task: String,
    args: Vec<String>,
}

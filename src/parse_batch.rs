use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::{config, Command, Scheduler};

pub fn parse_command(
    scheduler: &mut Scheduler,
    mut command_line: Vec<String>,
) -> Result<(), anyhow::Error> {
    let program = command_line.drain(1..).collect();
    create_command(scheduler, program, command_line)
}

pub fn parse_batch_file(scheduler: &mut Scheduler, file_name: String) -> Result<(), anyhow::Error> {
    let file = File::open(file_name)?;
    let file_buffered = BufReader::new(file);
    for line in file_buffered.lines() {
        if let Ok(line) = line {
            let line_trimmed = line.trim();
            if line_trimmed.starts_with("#") {
                continue;
            }
            let mut split = line.split_whitespace().map(|x| x.to_string());
            let program = split.next().unwrap();
            let args = split.collect();
            create_command(scheduler, program, args)?;
        }
    }
    Ok(())
}

fn create_command(
    scheduler: &mut Scheduler,
    mut program: String,
    args: Vec<String>,
) -> Result<(), anyhow::Error> {
    if program == config::EXECUTABLE {
        /* TODO execute tasks directly
        args.insert(0, config::EXECUTABLE.to_string());
        parse_cli(scheduler, args.into_iter())?
         */
        program = std::env::args_os().next().unwrap().into_string().unwrap();
        let command = Command::new_custom_command("".into(), program, args, vec![], vec![]);
        scheduler.push(Box::new(command));
    } else {
        let command = Command::new_custom_command("".into(), program, args, vec![], vec![]);
        scheduler.push(Box::new(command));
    }
    Ok(())
}

use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{process::Command, time::{Instant, Duration}, thread::sleep};

#[derive(Debug, Serialize, Deserialize)]
struct CommandExecutor {
    name: String,
    command: Vec<String>,
    exec_per_sec: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandExecutors {
    commands: Vec<CommandExecutor>,
}

fn main() {
    let f = std::fs::File::open("command_executor.yaml").expect("Could not open file");
    let command_execution: CommandExecutors =
        serde_yaml::from_reader(f).expect("Could not read values");

    for i in command_execution.commands.iter() {
        println!("Executing {} commands", i.name);
        execute_commands(&i.command, i.exec_per_sec);
    }
}

fn execute_commands(commands: &Vec<String>, ntimes: u32) {
    for i in commands {
        let args: Vec<&str> = i.split_whitespace().collect();
        let delay_ns = 1_000_000_000 / ntimes;
        let mut command_count = 0;


        loop{
            let start_time = Instant::now();
            command_count += 1;

            if command_count == ntimes {
                break;
            }

            let out = Command::new(args[0])
            .args(&args[1..=args.len() - 1])
            .output();


            
            match out {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    println!("Command output:\n{}", stdout);
                    println!("Command error:\n{}", stderr);
                    if out.status.success() {
                        println!(
                            "Command succeeded with exit code: {}",
                             out.status.code().unwrap_or(-1)
                        );
                    } else {
                        println!(
                            "Command failed with exit code: {}",
                             out.status.code().unwrap_or(-1)
                        );
                    }
                }
                Err(err) => {
                    eprintln!("Failed to execute command: {}", err);
                }
            }
            let elapsed_time = start_time.elapsed();

            let sleep_duration = if elapsed_time < Duration::from_nanos(delay_ns as u64) {
                Duration::from_nanos(delay_ns as u64) - elapsed_time
            } else {
                Duration::from_secs(0)
            };
            sleep(sleep_duration);
        }

    }
}

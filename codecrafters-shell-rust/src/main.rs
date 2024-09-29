#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{collections::HashSet, fs, io::BufRead};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let shell_built_ins: HashSet<&str> = HashSet::from(["echo", "exit", "type"]);
    let path_var = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = path_var.split(':').collect();
    loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input_split: Vec<&str> = input.split_whitespace().collect();
        let first_str = input_split[0];

        match first_str {
            "exit" => {
                std::process::exit(0);
            }
            "echo" => {
                if input_split.len() == 1 {
                    println!("");
                } else if input_split.len() > 1 {
                    println!("{}", input_split[1..].join(" "));
                }
            }
            "type" => {
                if input_split.len() > 2 {
                    let acc = input_split[1..].join(" ");
                    println!("{} not found", acc);
                } else if input_split.len() > 1 {
                    if shell_built_ins.contains(input_split[1]) {
                        println!("{} is a shell builtin", input_split[1]);

                        continue;
                    }

                    let mut command_loc: (bool, &str) = (false, "");
                    for &pth in paths.iter() {
                        let file = fs::File::open(pth).unwrap();
                        for line in std::io::BufReader::new(file).lines() {
                            if let Ok(line) = line {
                                if input_split[1] == line.as_str().trim() {
                                    command_loc = (true, pth);
                                    break;
                                }
                            }
                        }
                        if command_loc.0 {
                            break;
                        }
                    }

                    if command_loc.0 {
                        println!(
                            "{} is {}",
                            input_split[1],
                            (command_loc.1).to_string() + "/" + input_split[1]
                        );
                    } else {
                        println!("{}: command not found", input_split[1]);
                    }
                } else {
                    println!("");
                }
            }
            "pwd" => {
                if let Ok(val) = std::env::current_dir() {
                    println!("{}", val.display());
                }
            }
            _ => {
                Command::new(first_str)
                    .arg(input_split[1])
                    .status()
                    .expect("Failed to execute program");
            }
        }
    }
}

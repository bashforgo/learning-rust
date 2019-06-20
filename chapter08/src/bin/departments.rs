use std::collections::HashMap;
use std::io;

use lazy_static::lazy_static;
use regex::Regex;

enum Command {
    Add { person: String, department: String },
    List { department: String },
    Quit,
    Unknown(String),
}

fn main() {
    let mut departments = HashMap::new();
    loop {
        use Command::*;
        match parse_line() {
            Add { person, department } => {
                let had_department = departments.get(&department).is_some();
                println!(
                    "'{}' has been added to the{}department '{}'",
                    person,
                    if had_department { " " } else { " new " },
                    department
                );
                let people = departments.entry(department).or_insert(Vec::new());
                people.push(person);
                people.sort();
            }
            List { department } => match &department[..] {
                "*" => {
                    for entry in departments.iter() {
                        println!("department '{}':", entry.0);
                        for person in entry.1 {
                            println!("  {}", person);
                        }
                    }
                }
                _ => match departments.get(&department) {
                    Some(people) => {
                        println!("people assigned to '{}': {:?}", department, people);
                    }
                    None => {
                        println!(
                            "department not found, existing departments: {:?}",
                            departments.keys()
                        );
                    }
                },
            },
            Quit => break,
            Unknown(cmd) => println!("unknown command '{}'", cmd),
        }
    }
}

lazy_static! {
    static ref ADD_COMMAND_REGEX: Regex =
        Regex::new(r"(?i)^\s*add\s+(?P<name>.+)\s+to\s+(?P<dept>.+)\s*$").unwrap();
    static ref LIST_COMMAND_REGEX: Regex =
        Regex::new(r"(?i)^\s*list(\s+(?P<dept>.+))?\s*$").unwrap();
    static ref QUIT_COMMAND_REGEX: Regex = Regex::new(r"(?i)^\s*quit\s*$").unwrap();
}

fn parse_line() -> Command {
    use Command::*;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    let lower = input.to_ascii_lowercase();

    if ADD_COMMAND_REGEX.is_match(&lower) {
        match ADD_COMMAND_REGEX.captures(&lower) {
            Some(m) => {
                let person = String::from(m.name("name").unwrap().as_str());
                let department = String::from(m.name("dept").unwrap().as_str());
                Add { person, department }
            }
            None => Unknown(input),
        }
    } else if LIST_COMMAND_REGEX.is_match(&lower) {
        match LIST_COMMAND_REGEX.captures(&lower) {
            Some(m) => {
                let department = String::from(m.name("dept").map(|m| m.as_str()).unwrap_or("*"));
                List { department }
            }
            None => Unknown(input),
        }
    } else if QUIT_COMMAND_REGEX.is_match(&lower) {
        Quit
    } else {
        Unknown(input)
    }
}

mod patterns;
use clap::{Arg, ArgAction, Command};
use colored::*;
use libc::{STDIN_FILENO, isatty};
use patterns::regex::regex_match;
use patterns::simple::simple_pattern;
use regex::Regex;
use std::env;
use std::io::{self, Read};

// Input reader from the prefix of the program
fn read_input_command_prefix() -> Result<String, io::Error> {
    unsafe {
        if isatty(STDIN_FILENO as i32) == 0 {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        } else {
            Ok("Usage: grup [OPTION]... PATTERNS [FILE]...".to_string())
        }
    }
}

// Input reader from the suffix of the program
fn read_input_command_suffix() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    Ok(args.join(" "))
}

fn main() -> Result<(), io::Error> {
    let matches = Command::new("grup")
        .version("1.0")
        .author("Nithin V")
        .about("A grep clone written in rust")
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .action(ArgAction::SetTrue)
                .help("Ignore case distinctions in patterns"),
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("The pattern to search for")
                .required(true),
        )
        .get_matches();
    let input_prefix = read_input_command_prefix()?;
    let input_suffix = read_input_command_suffix()?;

    if input_suffix.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let ignore_case = matches.get_flag("ignore-case");

    let positions = if Regex::new(r"[.^$*+?()\\[\\]{}|\\]")
        .unwrap()
        .is_match(pattern)
    {
        // Regex pattern matching
        match regex_match(&input_prefix, pattern) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Invalid regex pattern: {}", e);
                return Ok(());
            }
        }
    } else {
        // Simple pattern matching
        simple_pattern(&input_prefix, pattern, !ignore_case)
    };

    if positions.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

    let mut sorted_matches = positions;
    sorted_matches.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = input_prefix.clone();

    for (pos, len) in sorted_matches {
        if pos + len <= result.len() {
            let before = &result[..pos];
            let middle = &result[pos..pos + len];
            let after = &result[pos + len..];

            result = format!("{}{}{}", before, middle.green(), after);
        }
    }

    println!("{}", result);
    Ok(())
}

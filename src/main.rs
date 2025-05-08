mod patterns;
use clap::{Arg, ArgAction, Command};
use colored::*;
use libc::{STDIN_FILENO, isatty};
use patterns::regex::regex_match;
use patterns::simple::simple_pattern;
use regex::Regex;
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
            Arg::new("only-matching")
                .short('o')
                .long("only-matching")
                .action(ArgAction::SetTrue)
                .help("show only the pattern matches"),
        )
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("The pattern to search for")
                .required(true),
        )
        .allow_hyphen_values(true)
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let ignore_case = matches.get_flag("ignore-case");
    let input_prefix = read_input_command_prefix()?;
    let input_suffix = pattern;

    if input_suffix.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

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

    if matches.get_flag("only-matching") {
        // Print each match individually
        for (pos, len) in sorted_matches {
            if pos + len <= result.len() {
                let middle = &result[pos..pos + len];
                println!("{}", middle.green());
            }
        }
    } else {
        // Highlight matches in the full string
        for (pos, len) in sorted_matches {
            if pos + len <= result.len() {
                let before: &str = &result[..pos];
                let middle: &str = &result[pos..pos + len];
                let after: &str = &result[pos + len..];
                result = format!("{}{}{}", before, middle.green(), after);
            }
        }
        println!("{}", result);
    }

    Ok(())
}

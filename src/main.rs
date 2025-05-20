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
            eprintln!("Error: No input provided. Usage: grup [OPTION]... PATTERNS [FILE]...");
            std::process::exit(1);
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
            Arg::new("max-count")
                .short('m')
                .long("max-count")
                .value_name("NUM")
                .action(ArgAction::Set)
                .help("stop after NUM selected lines"),
        )
        .arg(
            Arg::new("invert-match")
                .short('v')
                .long("invert-match")
                .help("select non-matching lines")
                .action(ArgAction::SetTrue),
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
    let _input_suffix = pattern;

    let positions = if Regex::new(r"[.^$*+?()\[\]{}|\\]")
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
        let positions: Vec<(usize, usize)> = if pattern.is_empty() {
            Vec::new()
        } else {
            // Simple pattern matching
            simple_pattern(&input_prefix, pattern, !ignore_case)
        };
        positions
    };

    if positions.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

    //Helper function to highlight the result
    fn highlighter_function(result: String, match_index: Vec<(usize, usize)>) -> String {
        let mut highlighted_result = result.clone();
        for (pos, len) in match_index {
            if pos + len <= highlighted_result.len() {
                let before: &str = &highlighted_result[..pos];
                let middle: &str = &highlighted_result[pos..(pos + len) as usize];
                let after: &str = &highlighted_result[(pos + len) as usize..];
                highlighted_result = format!("{}{}{}", before, middle.red().bold(), after);
            }
        }
        highlighted_result
    }

    let mut sorted_matches = positions;
    sorted_matches.sort_by(|a, b| b.0.cmp(&a.0));

    let result = input_prefix.clone();

    // Print only the matching pattern
    if matches.get_flag("only-matching") {
        for (pos, len) in sorted_matches {
            if pos + len <= result.len() {
                let middle = &result[pos..pos + len];
                println!("{}", middle.red().bold());
            }
        }
    }
    // Prints the number of line count specified with -m=NUM flag
    else if matches.contains_id("max-count") {
        if let Some(max_count_str) = matches.get_one::<String>("max-count") {
            if let Ok(max_count) = max_count_str.parse::<usize>() {
                let mut line_count = 1;
                for line in highlighter_function(result, sorted_matches).lines() {
                    if line_count <= max_count {
                        println!("{}", line);
                        line_count += 1;
                    } else {
                        {
                            break;
                        }
                    }
                }
            } else {
                eprintln!("Invalid max-count value: {}", max_count_str);
            }
        }
    }
    // Prints the non matching lines in the given input
    else if matches.get_flag("invert-match") {
        for line in highlighter_function(result, sorted_matches).lines() {
            if line.contains("\x1b[1;31m") && line.contains("\x1b[0m") {
                continue;
            } else {
                println!("{}", line);
            }
        }
    } else {
        // Prints the regular case sensitive match for given input
        for line in highlighter_function(result, sorted_matches).lines() {
            if line.contains("\x1b[1;31m") && line.contains("\x1b[0m") {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

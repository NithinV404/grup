mod patterns;
use colored::*;
use libc::{STDIN_FILENO, isatty};
use patterns::regex::regex_match;
use patterns::simple::simple_pattern;
use regex::Regex;
use std::env;
use std::io::{self, Read};

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

fn read_input_command_suffix() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    Ok(args.join(" "))
}

fn main() -> Result<(), io::Error> {
    let input_prefix = read_input_command_prefix()?;
    let input_suffix = read_input_command_suffix()?;

    if input_suffix.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

    let regex_metachar_pattern = Regex::new("[.^$*+?()\\[\\]{}|\\\\]").unwrap();
    let is_regex = regex_metachar_pattern.is_match(&input_suffix);

    let mut matches = Vec::new();

    if is_regex {
        match regex_match(&input_prefix, &input_suffix) {
            Ok(result) => matches = result,
            Err(e) => {
                eprintln!("Invalid regex pattern: {}", e);
                return Ok(());
            }
        }
    } else {
        let positions = simple_pattern(&input_prefix, &input_suffix);
        let pattern_len = input_suffix.len();
        for pos in positions {
            matches.push((pos, pattern_len));
        }
    }

    if matches.is_empty() {
        println!("{}", input_prefix);
        return Ok(());
    }

    matches.sort_by(|a, b| b.0.cmp(&a.0));

    let mut result = input_prefix.clone();

    for (pos, len) in matches {
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

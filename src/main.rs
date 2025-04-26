mod patterns;
use colored::*;
use libc::{STDIN_FILENO, isatty};
use patterns::simple::simple_pattern;
use std::env;
use std::io::{self, Read};

fn read_input_left() -> Result<String, io::Error> {
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

fn read_input_right() -> Result<String, io::Error> {
    let buffer = env::args().skip(1).collect();
    Ok(buffer)
}

fn main() -> Result<(), io::Error> {
    let input_left = read_input_left()?;
    let input_right = read_input_right()?;
    let indices = simple_pattern(&input_left, &input_right);
    let mut result = String::new();
    let mut current_pos = 0;
    let mut sorted_indices = indices.clone();
    sorted_indices.sort();
    for &pos in &sorted_indices {
        let start = pos as usize;
        let end = start + input_right.len();

        result.push_str(&input_left[current_pos..start]);

        result.push_str(&format!("{}", &input_left[start..end].yellow()));

        current_pos = end;
    }
    result.push_str(&input_left[current_pos..]);

    println!("{}", result);

    Ok(())
}

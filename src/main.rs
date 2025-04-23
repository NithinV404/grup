use libc::{STDIN_FILENO, isatty};
use std::io::{self, Read};

fn read_input() -> Result<String, io::Error> {
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
    let input = read_input()?;
    println!("{}", input);
    Ok(())
}

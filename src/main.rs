use std::io::{Read, Result};
use std::{env, io};

fn main() -> Result<()> {
    let mut count_bytes = false;
    let mut count_lines = false;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1_usize {
        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-c" => count_bytes = true,
                "-l" => count_lines = true,
                _ => (),
            }
        }
    }
    let mut buffer = String::new();
    let byte_count = io::stdin().read_to_string(&mut buffer)?;
    if count_bytes {
        print!("\t{byte_count}");
    }
    let line_count = buffer.lines().count();
    if count_lines {
        println!("\t{line_count}");
    }
    Ok(())
}

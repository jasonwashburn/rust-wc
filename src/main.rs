use std::io::{Read, Result};
use std::{env, io};

struct FlagOptions {
    count_lines: bool,
    count_bytes: bool,
}

impl FlagOptions {
    fn new() -> Self {
        FlagOptions {
            count_lines: false,
            count_bytes: false,
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let flag_options = match parse_args(args) {
        Ok(x) => x,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };
    let mut buffer = String::new();
    let byte_count = io::stdin().read_to_string(&mut buffer)?;
    if flag_options.count_bytes {
        print!("\t{byte_count}");
    }
    let line_count = buffer.lines().count();
    if flag_options.count_lines {
        print!("\t{line_count}");
    }
    println!();
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<FlagOptions> {
    let mut flag_options = FlagOptions::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-c" => flag_options.count_bytes = true,
            "-l" => flag_options.count_lines = true,
            _ => {
                return Err(std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unrecognized Argument: {arg}"),
                ))
            }
        }
    }
    Ok(flag_options)
}

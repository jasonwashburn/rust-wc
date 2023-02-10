use std::io::{Read, Result};
use std::{env, io};

struct FlagOptions {
    count_lines: bool,
    count_bytes: bool,
    count_words: bool,
}

impl FlagOptions {
    fn new() -> Self {
        FlagOptions {
            count_lines: false,
            count_bytes: false,
            count_words: false,
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
    let line_count = buffer.lines().count();
    let word_count = buffer
        .replace('\n', " ")
        .split(' ')
        .map(|w| w.trim())
        .filter(|w| w != &"")
        .count();
    if flag_options.count_lines {
        print!("{line_count:>8}");
    }
    if flag_options.count_words {
        print!("{word_count:>8}");
    }
    if flag_options.count_bytes {
        print!("{byte_count:>8}");
    }
    println!();
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<FlagOptions> {
    let mut flag_options = FlagOptions::new();
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-c" => flag_options.count_bytes = true,
                "-l" => flag_options.count_lines = true,
                "-w" => flag_options.count_words = true,
                _ => {
                    return Err(std::io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unrecognized Argument: {arg}"),
                    ))
                }
            }
        }
    } else {
        flag_options = FlagOptions {
            count_lines: true,
            count_bytes: true,
            count_words: true,
        };
    }
    Ok(flag_options)
}

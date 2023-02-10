use std::io::{Read, Result};
use std::{env, io};

#[derive(Default)]
struct RunConfig {
    count_lines: bool,
    count_bytes: bool,
    count_words: bool,
    files: Vec<String>,
}

impl RunConfig {
    fn new() -> Self {
        RunConfig {
            count_lines: false,
            count_bytes: false,
            count_words: false,
            ..Default::default()
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buffer = String::new();

    let run_config = match parse_args(args) {
        Ok(x) => x,
        Err(e) => {
            println!("{e}");
            std::process::exit(1);
        }
    };
    dbg!(&run_config.files);
    let byte_count = io::stdin().read_to_string(&mut buffer)?;

    let line_count = buffer.lines().count();
    let word_count = buffer.replace('\n', " ").split_whitespace().count();
    if run_config.count_lines {
        print!("{line_count:>8}");
    }
    if run_config.count_words {
        print!("{word_count:>8}");
    }
    if run_config.count_bytes {
        print!("{byte_count:>8}");
    }
    println!();
    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<RunConfig> {
    let mut run_config = RunConfig::new();
    let mut still_parsing_flags = true;
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            if arg.starts_with('-') && still_parsing_flags {
                if let Some(flag_chars) = arg.strip_prefix('-') {
                    run_config = parse_flags(flag_chars)?;
                }
            } else {
                still_parsing_flags = false;
                run_config.files.push(arg.to_owned());
            }
        }
        dbg!(&run_config.files);
    } else {
        run_config = RunConfig {
            count_lines: true,
            count_bytes: true,
            count_words: true,
            ..Default::default()
        };
    }
    Ok(run_config)
}

fn parse_flags(flag_chars: &str) -> Result<RunConfig> {
    let mut flag_options = RunConfig::new();
    for character in flag_chars.chars() {
        match character {
            'c' => flag_options.count_bytes = true,
            'l' => flag_options.count_lines = true,
            'w' => flag_options.count_words = true,
            _ => {
                return Err(std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unrecognized Argument: {character}"),
                ))
            }
        }
    }
    Ok(flag_options)
}

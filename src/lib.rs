use std::io;
use std::io::Result;

#[derive(Default)]
pub struct RunConfig {
    pub count_lines: bool,
    pub count_bytes: bool,
    pub count_words: bool,
    pub files: Vec<String>,
}

impl RunConfig {
    pub fn new() -> Self {
        RunConfig {
            count_lines: false,
            count_bytes: false,
            count_words: false,
            ..Default::default()
        }
    }

    fn no_flags_set(&self) -> bool {
        !self.count_bytes && !self.count_lines && !self.count_words
    }

    fn set_all_flags(&mut self, setting: bool) {
        self.count_words = setting;
        self.count_lines = setting;
        self.count_bytes = setting;
    }
}

pub struct Counts {
    byte_count: usize,
    word_count: usize,
    line_count: usize,
}

impl Counts {
    pub fn new() -> Self {
        Self {
            byte_count: 0,
            word_count: 0,
            line_count: 0,
        }
    }

    pub fn update(&mut self, other: Counts) {
        self.byte_count += other.byte_count;
        self.word_count += other.word_count;
        self.line_count += other.line_count;
    }
}

impl Default for Counts {
    fn default() -> Self {
        Self::new()
    }
}

pub fn count_and_output(buffer: String, run_config: &RunConfig) -> Counts {
    let mut counts = Counts::new();
    if run_config.count_lines {
        counts.line_count = buffer.lines().count();
        print!("{:>8}", counts.line_count);
    }
    if run_config.count_words {
        counts.word_count = buffer.replace('\n', " ").split_whitespace().count();
        print!("{:>8}", counts.word_count);
    }
    if run_config.count_bytes {
        counts.byte_count = buffer.len();
        print!("{:>8}", counts.byte_count);
    }
    counts
}

pub fn print_totals(run_config: &RunConfig, counts: Counts) {
    if run_config.count_lines {
        print!("{:>8}", counts.line_count);
    }
    if run_config.count_words {
        print!("{:>8}", counts.word_count);
    }
    if run_config.count_bytes {
        print!("{:>8}", counts.byte_count);
    }
    println!(" total")
}

pub fn parse_args(args: Vec<String>) -> Result<RunConfig> {
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
        if run_config.no_flags_set() {
            run_config.set_all_flags(true);
        }
    } else {
        run_config.set_all_flags(true);
    }
    Ok(run_config)
}

pub fn print_usage() {
    println!("usage: wc [-clmw] [file ...]");
}

pub fn parse_flags(flag_chars: &str) -> Result<RunConfig> {
    let mut flag_options = RunConfig::new();
    for character in flag_chars.chars() {
        match character {
            'c' => flag_options.count_bytes = true,
            'l' => flag_options.count_lines = true,
            'w' => flag_options.count_words = true,
            _ => {
                return Err(std::io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("wc: illegal option -- {character}"),
                ))
            }
        }
    }
    Ok(flag_options)
}

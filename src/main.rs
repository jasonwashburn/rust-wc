use std::io::{Read, Result};
use std::{env, fs, io};

use rust_wc::{count_and_output, parse_args, print_totals, print_usage, Counts};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut buffer = String::new();

    let run_config = match parse_args(args) {
        Ok(x) => x,
        Err(e) => {
            println!("{e}");
            print_usage();
            std::process::exit(1);
        }
    };
    if run_config.files.is_empty() {
        io::stdin().read_to_string(&mut buffer)?;
        count_and_output(buffer, &run_config);
        println!();
    } else {
        let mut counts = Counts::new();
        for file in &run_config.files {
            match fs::read_to_string(file) {
                Ok(s) => {
                    let buffer = s;
                    let new_counts = count_and_output(buffer, &run_config);
                    counts.update(new_counts);
                    print!(" {file}");
                    println!();
                }
                Err(e) => println!("wc: {file}: read: {e}"),
            }
        }
        print_totals(&run_config, counts);
    }
    Ok(())
}

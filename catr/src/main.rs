use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                let mut prev_number = 0;
                for (line_number, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if args.number_lines {
                        println!("{:6}\t{}", line_number + 1, line);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_number += 1;
                            println!("{:6}\t{}", prev_number, line);
                        }
                    } else {
                        println!("{line}")
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

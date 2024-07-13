#![allow(unused)]

use clap::Arg;
use clap::Parser;
use kdam::tqdm;
use kdam::BarExt;
use std::env::args;
use std::thread;
use std::sync::mpsc;
use std::fs;
use csv::WriterBuilder;
use std::error::Error;
use rand::seq::SliceRandom;

/// Program to generate data
#[derive(Parser, Debug, Clone)]
#[command(about, long_about = None)]
struct Args {
    /// File to read from.
    /// Do include file extensions.
    #[arg(short, long, default_value_t = String::from("data"))]
    input: String,

    /// Filename to write to
    /// Don't include file extensions 
    #[arg(short, long, default_value_t = String::from("data"))]
    output: String,
}

fn check_inputs(args: &Args) {
    // nothing for now
}

fn get_chess_lines(input: String, args: &Args) -> Vec<String> {
    let chess_lines: Vec<String> = Vec::new();

    for line in input.split("\n") {
        println!("{}", line);
        assert!(0==1, "lmao");
    }

    return chess_lines;
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // throw error if inputs are bad
    check_inputs(&args);

    let input = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");

    let chess_lines = get_chess_lines(input, &args);

    Ok(())
}
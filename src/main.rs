use std::fs;

use clap::Parser;

use crate::parser::ttf_parser;

mod parser;
mod tag;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ttf_file: String,
}

fn main() {
    let args = Args::parse();

    let data = fs::read(args.ttf_file).unwrap();
    ttf_parser(&data).unwrap();
}

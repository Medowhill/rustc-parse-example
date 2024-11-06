use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    rustc_parse_example::compiler::run(&args.input);
}

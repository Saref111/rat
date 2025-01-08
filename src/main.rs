use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(name = "Rat")]
#[command(version, about = "Naive implementation of cat CLI command in Rust", long_about = None)]
struct Cli {
    file: Option<PathBuf>,

    ///equivalent to -vET
    #[arg(short = 'a', long)]
    show_all: bool,

    ///number nonempty output lines, overrides -n
    #[arg(short = 'b', long)]
    number_nonblank: bool,

    ///equivalent to -vE
    #[arg(short)]
    e: bool,

    ///equivalent to -vE
    #[arg(short = 'E', long)]
    show_ends: bool,

    ///number all output lines
    #[arg(short, long)]
    number: bool,

    ///suppress repeated empty output lines
    #[arg(short, long)]
    squeeze_blank: bool,

    ///equivalent to -vT
    #[arg(short)]
    t: bool,

    ///display TAB characters as ^I
    #[arg(short = 'T', long)]
    show_tabs: bool,

    ///use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long)]
    show_nonprinting: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.file {
        Some(filename) => from_file(filename, &cli),
        None => from_std_input(cli),
    } 
}

fn from_file(filename: &PathBuf, cli: &Cli) {
    let Ok(mut file_content) = File::open(filename) else {
        panic!("rat: {}: No such file or directory", filename.to_string_lossy());
    };

    let mut file_string = String::new();

    let Ok(_) = file_content.read_to_string(&mut file_string) else {
        panic!("rat: {}: Cannot read the file", filename.to_string_lossy());
    };

    println!("{file_string}");
}

fn from_std_input(cli: Cli) {
    todo!()
}
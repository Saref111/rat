mod process;
use std::{fs::File, io::Read, path::PathBuf};
use process::{
    enumerate_lines,
    suppress_empty_lines,
};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Rat")]
#[command(version, about = "Naive implementation of cat CLI command in Rust", long_about = None)]
struct Cli {
    file: Option<PathBuf>, // done

    ///equivalent to -vET
    #[arg(short = 'a', long)]
    show_all: bool,

    ///number nonempty output lines, overrides -n
    #[arg(short = 'b', long)]
    number_nonblank: bool, // done

    ///equivalent to -vE
    #[arg(short)]
    e: bool,

    ///display $ at end of each line
    #[arg(short = 'E', long)]
    show_ends: bool, // done

    ///number all output lines
    #[arg(short, long)]
    number: bool, // done

    ///suppress repeated empty output lines
    #[arg(short, long)]
    squeeze_blank: bool, // done

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

    let mut lines: Vec<String> = file_string.lines().map(|s| s.to_owned()).collect();

    if cli.squeeze_blank {
        lines = suppress_empty_lines(lines);
    }

    if cli.number || cli.number_nonblank {
        lines = enumerate_lines(lines, cli.number_nonblank);
    }

    if cli.show_ends {
        lines = lines.into_iter().map(|l| l + "$").collect();
    }

    for line in lines {
        println!("{line}");
    }
}

fn from_std_input(cli: Cli) {
    todo!()
}


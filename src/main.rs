mod process;
use std::{fs::File, io::Read, path::PathBuf};
use process::{process, ProcessingArgs};
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

    ///display $ at end of each line
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
    let args = ProcessingArgs {
        show_nonprinting: cli.show_nonprinting || cli.t || cli.show_all || cli.e,
        squeeze_blank: cli.squeeze_blank,
        number: cli.number,
        number_nonblank: cli.number_nonblank,
        show_ends: cli.show_ends || cli.e || cli.show_all,
        show_tabs: cli.show_tabs || cli.show_all,
    };

    match &cli.file {
        Some(filename) => from_file(filename, args),
        None => from_std_input(args),
    } 
}

fn from_file(filename: &PathBuf, args: ProcessingArgs) {
    let Ok(mut file_content) = File::open(filename) else {
        panic!("rat: {}: No such file", filename.to_string_lossy());
    };

    let mut file_string = String::new();

    let Ok(_) = file_content.read_to_string(&mut file_string) else {
        panic!("rat: {}: Cannot read the file", filename.to_string_lossy());
    };

    let lines = process(file_string, args);

    print_lines(lines);
}

fn from_std_input(args: ProcessingArgs) {
    
    loop {
        let local_args = &args;
        let mut input = String::new();
        std::io::stdin()
        .read_line(&mut input)
        .expect("rat: Failed to read line");
    
        let lines = process(input, *local_args);
    
        print_lines(lines);
    }
}

fn print_lines(lines: Vec<String>) {
    for line in lines {
        println!("{line}");
    }
}  
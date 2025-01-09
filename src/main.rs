mod process;
use std::{fs::File, io::Read};
use process::{process, ProcessingArgs};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Rat")]
#[command(version, about = "Naive implementation of cat CLI command in Rust", long_about = None)]
struct Cli {
    files: Option<Vec<String>>, 

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

    match &cli.files {
        Some(files) => files.into_iter().for_each(|filename| {
            if filename == "-" {
                from_std_input(args)
            } else {
                from_file(filename.to_string(), args)
            }
        }),
        None => from_std_input(args),
    } 
}

fn from_file(filename: String, args: ProcessingArgs) {
    let mut file_content = exit_on_error(File::open(&filename), &filename);
    let mut file_string = String::new();

    exit_on_error(file_content.read_to_string(&mut file_string), &filename);

    let lines = process(file_string, args);

    print_lines(lines);
}

fn from_std_input(args: ProcessingArgs) {
    
    loop {
        let mut input = String::new();

        let bytes = std::io::stdin()
            .read_line(&mut input)
            .expect("rat: Failed to read line");

        let lines = process(input, args);
        
        if bytes == 0 {
            break;
        }

        print_lines(lines);
    }
}

fn print_lines(lines: Vec<String>) {
    lines.iter().for_each(|line| println!("{line}"));
} 

fn exit_on_error<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> T {
    result.unwrap_or_else(|err| {
        eprintln!("rat: {}: {}", context, err);
        std::process::exit(1);
    })
}
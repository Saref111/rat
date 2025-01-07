use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Rat")]
#[command(version, about, long_about = None)]
struct Cli {
    file: Option<String>,

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

    if cli.show_all {
        println!("show all");
    }
}
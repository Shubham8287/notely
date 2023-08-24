use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Operations>,
}

#[derive(Subcommand)]
enum Operations {
    Add {
        name: Option<String>,
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    View {
        #[arg(short, long)]
        list: bool,
    }
}
fn main() {
    let cli = Cli::parse();
    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Operations::Add { name, list }) => {
            if *list {
                println!("Printing testing lists... {:?}", *name);
            } else {
                println!("Not printing testing lists...");
            }
        },
        Some(Operations::View { list }) => {
            if *list {
                println!("View testing lists...");
            } else {
                println!("Not View testing lists...");
            }
        }
        None => {}
    }
}

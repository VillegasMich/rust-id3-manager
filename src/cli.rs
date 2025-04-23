use clap::*;
use colored::Colorize;
use std::io;

use crate::commands_manager::CommandsManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Rust ID3 tags manager")]
pub struct Args {
    /// Path to the audio file
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Show supported ID3 tags
    #[arg(long)]
    pub supp_tags: bool,

    /// Show ID3 tags
    #[arg(short, long, requires = "file")]
    pub show: bool,

    /// Add ID3 tag
    #[arg(short, long, value_name = "\"ID3-TAG=VALUE\"", requires = "file")]
    pub add: Option<String>,
}

pub fn parse() -> io::Result<()> {
    println!();
    let cli = Args::parse();
    if cli.supp_tags {
        CommandsManager::show_supported_id3_tags();
        return Ok(());
    }
    match (&cli.file, cli.show, &cli.add) {
        (Some(file), true, _) => CommandsManager::show(file),
        (Some(file), false, Some(tag)) => CommandsManager::add(file, tag),
        (Some(file), false, None) => {
            println!("{} {}", "File selected:".bold(), file.underline());
            println!("No other command found.");
            println!(
                "Use {} to display all available commands.",
                "--help".yellow().bold()
            );
            Ok(())
        }
        (None, _, _) => {
            eprintln!(
                "❌ {}",
                "No file provided. Use -f <FILE> to specify an audio file."
                    .red()
                    .bold()
            );
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No file provided",
            ))
        }
    }
}

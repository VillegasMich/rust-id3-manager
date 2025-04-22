use crate::audio_file;
use audio_file::AudioFileWithTags;
use clap::*;
use std::{io, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Rust ID3 tags manager")]
pub struct Args {
    /// Path to the audio file
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<String>,

    /// Show ID3 tags
    #[arg(short, long)]
    pub show: bool,
}

fn show(file: &str) -> io::Result<()> {
    let file_path = PathBuf::from(file);
    println!("Attempting to parse: {:?}", file_path.display());
    match AudioFileWithTags::from_path(&file_path) {
        Ok(audio_file) => {
            audio_file.display_tags();
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to process file {:?}: {}", file_path.display(), e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}

pub fn parse() -> io::Result<()> {
    let cli = Args::parse();
    match (&cli.file, cli.show) {
        (Some(file), true) => show(file),
        (Some(file), false) => {
            println!("File selected: {}", file);
            println!("No other command found");
            println!("Use --help to display all teh commands.");
            Ok(())
        }
        (None, _) => {
            eprintln!("❌ No file provided. Use -f <FILE> to specify an audio file.");
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No file provided",
            ))
        }
    }
}

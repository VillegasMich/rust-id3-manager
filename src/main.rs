use std::io;

pub mod audio_file;
pub mod cli;

fn main() -> io::Result<()> {
    cli::parse()
}

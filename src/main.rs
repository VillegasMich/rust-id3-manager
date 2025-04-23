use std::io;

pub mod audio_file;
pub mod cli;
pub mod commands_manager;

fn main() -> io::Result<()> {
    cli::parse()
}

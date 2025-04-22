use std::env;
use std::path::PathBuf;
use std::process;

// Declare the module defined in audio_file.rs
pub mod audio_file;

// Bring the public items from the module into scope
use audio_file::AudioFileWithTags;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_audio_file>", args[0]);
        process::exit(1);
    }

    let file_path_str = &args[1];
    let file_path = PathBuf::from(file_path_str);

    println!("Attempting to parse: {:?}", file_path.display());

    match AudioFileWithTags::from_path(&file_path) {
        Ok(audio_file) => {
            audio_file.display_tags();
        }
        Err(e) => {
            eprintln!("Failed to process file {:?}: {}", file_path.display(), e);
            process::exit(1); // Exit with a non-zero status code to indicate failure
        }
    }
}

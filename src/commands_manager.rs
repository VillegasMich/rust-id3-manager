use crate::audio_file::AudioFileWithTags;
use std::{io, path::PathBuf};

use colored::Colorize;
pub struct CommandsManager {}

impl CommandsManager {
    pub fn show(file: &str, as_json: bool) -> io::Result<()> {
        let file_path = PathBuf::from(file);
        println!("Attempting to parse: {:?}", file_path.display());
        match AudioFileWithTags::from_path(&file_path) {
            Ok(audio_file) => {
                if as_json {
                    audio_file.display_as_json();
                } else {
                    audio_file.display_tags();
                }
                println!(
                    "{}",
                    format!("✅ Parsing successfull for file {:?}", file_path)
                        .green()
                        .bold()
                );
                Ok(())
            }
            Err(e) => {
                eprintln!(
                    "❌ {} {:?}: {}",
                    "Failed to process file".red().bold(),
                    file_path.display(),
                    e
                );
                Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        }
    }

    pub fn add(file: &str, tag_value: &str) -> io::Result<()> {
        let file_path = PathBuf::from(file);
        let (key, value) = match tag_value.split_once('=') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => {
                eprintln!("❌ {}", "Invalid tag format. Use KEY=VALUE.".red().bold());
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid tag format",
                ));
            }
        };
        let mut audio_with_tags = match AudioFileWithTags::from_path(&file_path) {
            Ok(audio_with_tags) => audio_with_tags,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
        };

        match audio_with_tags.add_tag(key, value) {
            Ok(()) => {
                println!(
                    "{}",
                    format!("✅ Added tag '{}={}' to file {:?}", key, value, file_path)
                        .green()
                        .bold()
                );
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ {} {}", "Failed to add tag:".red().bold(), e);
                Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
            }
        }
    }

    pub fn show_supported_id3_tags() {
        let supported_tags = [
            ("TIT2", "Title"),
            ("TPE1", "Artist"),
            ("TALB", "Album"),
            ("TYER", "Year"),
            ("TRCK", "Track number"),
            ("TCON", "Genre"),
            ("COMM", "Comment"),
            ("TXXX", "User-defined text information"),
            ("USLT", "Lyrics"),
            ("TENC", "Encoded by"),
            ("TSSE", "Software/Hardware encoder"),
            ("TCOM", "Composer"),
            ("TPOS", "Part of a set (disc number)"),
            ("WXXX", "User-defined URL link"),
            ("APIC", "Attached picture"),
            ("TOPE", "Original artist"),
            ("TORY", "Original release year"),
            ("TIT3", "Subtitle"),
            ("TLEN", "Length (duration)"),
            ("TKEY", "Initial key"),
            ("TEXT", "Lyricist"),
            ("TFLT", "File type"),
            ("TSRC", "ISRC code"),
            ("WCOM", "Commercial URL"),
            ("WOAR", "Artist URL"),
            ("TPUB", "Publisher"),
            ("TOAL", "Original album"),
            ("TCOP", "Copyright"),
            ("TPE2", "Band/Orchestra"),
            ("TPE3", "Conductor"),
            ("TPE4", "Interpreted by"),
            ("TOFN", "Original filename"),
        ];

        println!("{}", "\nSupported ID3 Tag IDs:\n".bold());
        for (id, description) in supported_tags.iter() {
            println!("  {} => {}", id, description);
        }
        println!(
            "\nUse the tag {} as the key when adding a tag (e.g. {}).\n",
            "ID".bold().yellow(),
            "TIT2=My Song".bold()
        );
    }
}

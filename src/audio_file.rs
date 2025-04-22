use id3::{Error, ErrorKind, Tag, TagLike}; // Use alias for clarity
use std::path::{Path, PathBuf};
use std::result::Result as StdResult; // Alias for standard Result

pub struct AudioFileWithTags {
    path: PathBuf,
    tags: Option<Tag>,
}

impl AudioFileWithTags {
    pub fn from_path<P: AsRef<Path>>(path: P) -> StdResult<Self, Error> {
        let path_buf = path.as_ref().to_path_buf();

        // Use the id3 crate's read_from_path function
        match Tag::read_from_path(&path_buf) {
            Ok(tag) => {
                // Successfully read a tag
                println!(
                    "Info: Successfully read ID3 tag from {:?}",
                    path_buf.display()
                );
                Ok(AudioFileWithTags {
                    path: path_buf,
                    tags: Some(tag),
                })
            }
            Err(e) => {
                if let ErrorKind::NoTag = e.kind {
                    // This is the NoTag error
                    println!("Info: No ID3 tag found for {:?}", path_buf.display());
                    Ok(AudioFileWithTags {
                        path: path_buf,
                        tags: None,
                    })
                } else {
                    eprintln!("Error reading ID3 tag for {:?}: {}", path_buf.display(), e);
                    Err(e)
                }
            }
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn tags(&self) -> &Option<Tag> {
        &self.tags
    }

    pub fn display_tags(&self) {
        println!(
            "--- Parsing results for {:?} ---",
            self.path.file_name().unwrap_or_default()
        );
        match &self.tags {
            Some(tag) => {
                println!("{:#?}", tag);
                println!("  Status: ID3 Tag found");
                println!("  Title: {}", tag.title().unwrap_or("N/A"));
                println!("  Artist: {}", tag.artist().unwrap_or("N/A"));
                println!("  Album: {}", tag.album().unwrap_or("N/A"));
                // Use map_or for optional numerical values
                println!(
                    "  Year: {}",
                    tag.year().map_or("N/A".to_string(), |y| y.to_string())
                );
                println!("  Genre: {}", tag.genre().unwrap_or("N/A"));
                println!(
                    "  Track: {}",
                    tag.track().map_or("N/A".to_string(), |t| t.to_string())
                );
                println!(
                    "  Duration: {}",
                    tag.duration()
                        .map_or("N/A".to_string(), |d| format!("{}s", d))
                );
                // Add other fields as needed
            }
            None => {
                println!("  Status: No ID3 tag found");
            }
        }
        println!("------------------------------------------");
    }
}

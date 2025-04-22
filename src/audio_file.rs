use id3::{Error, ErrorKind, Tag};
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;

pub struct AudioFileWithTags {
    path: PathBuf,
    tags: Option<Tag>,
}

impl AudioFileWithTags {
    pub fn from_path<P: AsRef<Path>>(path: P) -> StdResult<Self, Error> {
        let path_buf = path.as_ref().to_path_buf();

        match Tag::read_from_path(&path_buf) {
            Ok(tag) => {
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
            "\n--- Parsing results for {:?} ---",
            self.path.file_name().unwrap_or_default()
        );
        match &self.tags {
            Some(tag) => {
                for frame in tag.frames() {
                    let content = if frame.content().to_string().is_empty() {
                        "N/A".to_string()
                    } else {
                        frame.content().to_string()
                    };
                    println!(" {} - {}: {}", frame.id(), frame.name(), content);
                }
            }
            None => {
                println!("  Status: No ID3 tag found");
            }
        }
        println!("------------------------------------------\n");
    }
}

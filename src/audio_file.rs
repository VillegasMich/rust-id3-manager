use colored::Colorize;
use id3::{Content, Error, ErrorKind, Frame, Tag, TagLike, Version};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
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
                    "{} {:?}",
                    "Info: Successfully read ID3 tag from".yellow(),
                    path_buf.display()
                );
                Ok(AudioFileWithTags {
                    path: path_buf,
                    tags: Some(tag),
                })
            }
            Err(e) => {
                if let ErrorKind::NoTag = e.kind {
                    println!(
                        "{} {:?}",
                        "Info: No ID3 tag found for".yellow(),
                        path_buf.display()
                    );
                    Ok(AudioFileWithTags {
                        path: path_buf,
                        tags: None,
                    })
                } else {
                    eprintln!(
                        "{} {:?} {}",
                        "Error reading ID3 tag for".red(),
                        path_buf.display(),
                        e
                    );
                    Err(e)
                }
            }
        }
    }

    pub fn add_tag(&mut self, id: &str, value: &str) -> StdResult<(), Error> {
        let tag = self.tags.get_or_insert_with(Tag::new);
        tag.add_frame(Frame::with_content(id, Content::Text(value.to_string())));
        tag.write_to_path(&self.path, Version::Id3v24)?;
        Ok(())
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
                    println!(" {} - {}: {}", frame.id().bold(), frame.name(), content);
                }
            }
            None => {
                println!("  Status: No ID3 tag found");
            }
        }
        println!("------------------------------------------\n");
    }

    pub fn display_as_json(&self) {
        match &self.tags {
            Some(tag) => {
                let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();

                for frame in tag.frames() {
                    let id = frame.id().to_string();
                    let content = frame.content().to_string();
                    let entry = tag_map.entry(id).or_default();
                    entry.push(if content.is_empty() {
                        "N/A".to_string()
                    } else {
                        content
                    });
                }

                let mut json_map = Map::new();
                for (k, v) in tag_map {
                    json_map.insert(k, json!(v));
                }

                println!(
                    "{}",
                    serde_json::to_string_pretty(&Value::Object(json_map)).unwrap()
                );
            }
            None => {
                println!("{}", json!({ "status": "No ID3 tag found" }));
            }
        }
    }
}

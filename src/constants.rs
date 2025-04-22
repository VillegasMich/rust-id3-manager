use once_cell::sync::Lazy;
use std::collections::HashMap;

static FRAME_LABELS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("TIT2", "Title");
    m.insert("TPE1", "Artist");
    m.insert("TALB", "Album");
    m.insert("TYER", "Year");
    m.insert("TCON", "Genre");
    m.insert("TRCK", "Track");
    m.insert("TDRC", "Recording Time");
    m.insert("TSSE", "Encoder");
    m.insert("TXXX", "User-defined Text");
    m.insert("COMM", "Comment");
    m
});

pub fn frame_id_to_label(id: &str) -> &str {
    FRAME_LABELS.get(id).copied().unwrap_or(id)
}

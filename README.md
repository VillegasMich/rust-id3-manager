# 🎵 ID3 Tag Inspector

A small CLI tool written in Rust to inspect and edit ID3 tags of MP3 audio files using the [`id3`](https://docs.rs/id3/latest/id3/) crate.

## ✨ Features

- Display all ID3v2 tags in a given `.mp3` file.
- Add new tags to the file.
- List all supported tag IDs with descriptions.
- Command-line interface built with [`clap`](https://docs.rs/clap/latest/clap/).

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/VillegasMich/rust-id3-manager
cd rust-id3-manager

# Build the project
cargo build --release

# Run the binary
./target/release/id3-parser --help

# Or build into the path
cargo install --path .

# Run from path
id3-parser --help
```

## 🚀 Usage

### Show tags from a file

```bash
id3-parser -f <file.mp3> --show
```

### Add a tag to a file

```bash
id3-parser -f <file.mp3> --add "TIT2=My Custom Title"
```

### Show supported tag IDs

```bash
id3-parser --supp-tags
```

## 🧠 Supported ID3 Tags (v2.3)

| Tag ID | Description                                  |
| ------ | -------------------------------------------- |
| TIT2   | Title                                        |
| TPE1   | Artist                                       |
| TALB   | Album                                        |
| TYER   | Year                                         |
| TRCK   | Track number                                 |
| TCON   | Genre                                        |
| COMM   | Comment                                      |
| TXXX   | User defined text information                |
| USLT   | Unsynchronized lyrics/text                   |
| TENC   | Encoded by                                   |
| TSSE   | Software/Hardware encoder                    |
| TCOM   | Composer                                     |
| TPOS   | Part of a set (disc number)                  |
| WXXX   | User defined URL link                        |
| APIC   | Attached picture                             |
| TOPE   | Original artist                              |
| TORY   | Original release year                        |
| TIT3   | Subtitle                                     |
| TLEN   | Length (duration)                            |
| TKEY   | Initial key                                  |
| TEXT   | Lyricist                                     |
| TFLT   | File type                                    |
| TSRC   | ISRC (International Standard Recording Code) |
| WCOM   | Commercial URL                               |
| WOAR   | Artist URL                                   |
| TPUB   | Publisher                                    |
| TOAL   | Original album                               |
| TCOP   | Copyright                                    |
| TPE2   | Band/Orchestra/Accompaniment                 |
| TPE3   | Conductor                                    |
| TPE4   | Interpreted by                               |
| TOFN   | Original filename                            |

## 🧩 Project Structure

- `src/cli.rs` - Clap CLI argument parsing.
- `src/audio_file.rs` - Logic for reading, writing, and displaying ID3 tags.
- `src/main.rs` - Entry point and command matching.
- `src/commands.rs` - Commands manager with `show`, `add`, and `supp_tags` functions.
- `resources/*` - .mp3 files for testing

## 🔧 Roadmap

- [ ] Add support for removing tags.
- [x] Add JSON export of tag data.
- [x] Add better terminal UI
- [ ] Add AI support for completing tags information

## 📝 License

...

---

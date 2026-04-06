use crate::metadata;
use crate::metadata::Metadata;
use anyhow::{Context, Result};
use chrono::Local;
use pulldown_cmark::{Event, Options, Parser, TagEnd, html};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Note {
    pub title: String,
    pub body: String,
    pub path: PathBuf,
    pub html: String,
    pub plain_text: String,
    pub metadata: Option<Metadata>,
}

/// Creates a new .md file at `dir/title.md` with the given body.
pub fn create_note(dir: &Path, title: &str, body: &str, tags: Vec<String>) -> Result<Note> {
    fs::create_dir_all(dir).context("Failed to create notes directory")?;

    let created = Local::now().format("%Y-%m-%d").to_string();
    let tags_yaml = tags
        .iter()
        .map(|tag| format!("\"{}\"", tag))
        .collect::<Vec<_>>()
        .join(", ");

    let raw = format!(
        "---\ntitle: \"{title}\"\ntags: [{tags_yaml}]\ncreated: \"{created}\"\n---\n\n# {title}\n\n{body}"
    );

    let path = dir.join(format!("{title}.md"));
    fs::write(&path, &raw)
        .with_context(|| format!("Failed to write note to {}", path.display()))?;

    let metadata = Metadata {
        title: Some(title.to_string()),
        tags,
        created: Some(created),
    };

    Ok(Note {
        title: title.to_string(),
        body: body.to_string(),
        path,
        html: to_html(body),
        plain_text: to_plain_text(body),
        metadata: Some(metadata),
    })
}

/// Reads a single .md file and returns a Note.
pub fn load_note(path: &Path) -> Result<Note> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("Failed to read note from {}", path.display()))?;

    let (metadata, body) = metadata::parse_front_matter(&raw)?;

    let title = metadata
        .as_ref()
        .and_then(|m| m.title.as_deref())
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
        })
        .to_string();

    let html = to_html(&body);
    let plain_text = to_plain_text(&body);

    Ok(Note {
        title,
        body,
        path: path.to_path_buf(),
        html,
        plain_text,
        metadata,
    })
}

/// Walks `dir` recursively and loads every .md file found.
pub fn load_all_notes(dir: &Path) -> Result<Vec<Note>> {
    let mut notes = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "md"))
    {
        match load_note(entry.path()) {
            Ok(note) => notes.push(note),
            Err(err) => eprintln!("Skipping note: {}: {err}", entry.path().display()),
        }
    }
    Ok(notes)
}

/// Renders Markdown source to an HTML string.
pub fn to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Extracts plain text from Markdown (strips all formatting).
/// Useful for search indexing and snippets.
pub fn to_plain_text(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut text = String::new();

    for event in parser {
        match event {
            Event::Text(s) | Event::Code(s) => {
                text.push_str(&s);
                text.push(' ');
            }
            Event::SoftBreak | Event::HardBreak => text.push('\n'),
            Event::End(TagEnd::Paragraph | TagEnd::Heading(_)) => text.push('\n'),
            _ => {}
        }
    }
    text.trim().to_string()
}

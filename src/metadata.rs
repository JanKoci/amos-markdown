use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub title: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created: Option<String>,
}

/// Splits a note file's content into (Option<Metadata>, body).
/// If the file starts with `---`, everything up to the closing `---` is
/// treated as YAML front matter. Otherwise the whole string is the body.
pub fn parse_front_matter(raw: &str) -> Result<(Option<Metadata>, String)> {
    if !raw.starts_with("---") {
        return Ok((None, raw.to_string()));
    }

    let rest = &raw[3..];
    let Some(closing_pos) = rest.find("\n---") else {
        return Ok((None, raw.to_string()));
    };
    let yaml = &rest[..closing_pos];
    let body = rest[closing_pos + 4..].trim_start().to_string();

    let metadata: Metadata = serde_yaml::from_str(yaml)?;
    Ok((Some(metadata), body))
}

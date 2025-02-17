use log::error;
use serde_json::Value;
use std::{
    collections::HashSet,
    process::{Command, Stdio},
};

pub fn has_suffix(name: &str, suffix: &str) -> bool {
    let name = name.trim_end_matches('.');
    let mut suffix = suffix.trim_end_matches('.');
    suffix = suffix.trim_start_matches('.');
    let name_base = name.trim_end_matches(suffix);
    name_base.len() < name.len() && name_base.ends_with('.')
}

pub fn trim_suffix(name: &str, suffix: &str) -> String {
    let mut new_name = name;
    if has_suffix(name, suffix) {
        let suffix = suffix.trim_start_matches('.').trim_end_matches('.');
        new_name = new_name.trim_end_matches('.');
        new_name = new_name.trim_end_matches(suffix);
    }
    new_name.trim_end_matches('.').to_string()
}

pub fn sanitize_hostname(hostname: &str) -> String {
    const MAX_LABEL_LENGTH: usize = 63;

    // Trim suffixes
    let hostname = hostname
        .trim_end_matches(".local")
        .trim_end_matches(".localdomain")
        .trim_end_matches(".lan");

    // Find the first/last alphanumeric characters
    let start = hostname.find(|c: char| c.is_alphanumeric()).unwrap_or(0);
    let end = hostname
        .rfind(|c: char| c.is_alphanumeric())
        .map_or(0, |e| e + 1);

    let separators: HashSet<char> = [' ', '.', '@', '_'].into();

    let mut sanitized: String = hostname[start..end]
        .chars()
        .enumerate()
        .map(|(index, char)| {
            let boundary = (index == 0) || (index == end - start - 1);
            if !boundary && separators.contains(&char) {
                '-'
            } else if char.is_alphanumeric() || char == '-' {
                char.to_ascii_lowercase()
            } else {
                char
            }
        })
        .collect();

    sanitized.truncate(MAX_LABEL_LENGTH);
    sanitized
}

// TODO: maybe properly deserialize the JSON?
pub fn check_tailscale_operator(user: &str) -> bool {
    if let Ok(output) = Command::new("tailscale")
        .arg("debug")
        .arg("prefs")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
    {
        if output.status.success() {
            let Ok(prefs) = serde_json::from_slice::<Value>(&output.stdout) else {
                error!("Failed to parse JSON");
                return false;
            };
            if let Some(operator) = prefs.get("OperatorUser") {
                return operator.as_str() == Some(user);
            }
        }
    }
    false
}

use serde::{Deserialize, Serialize};

pub mod articles;
pub mod canon;
pub mod catechism;
pub mod confession;
pub mod creed;
pub mod theses;

/// A scripture proof reference, shared across Creeds.json document types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub id: u32,
    pub references: Vec<String>,
}

/// Parse proofs from a Creeds.json JSON value.
pub fn parse_proofs(value: &serde_json::Value) -> Vec<Proof> {
    value["Proofs"]
        .as_array()
        .map(|proofs| {
            proofs
                .iter()
                .filter_map(|p| {
                    let id = p["Id"].as_u64()? as u32;
                    let refs = p["References"]
                        .as_array()?
                        .iter()
                        .filter_map(|r| r.as_str().map(String::from))
                        .collect();
                    Some(Proof {
                        id,
                        references: refs,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Format proofs as a human-readable string for feed descriptions.
pub fn format_proofs(proofs: &[Proof]) -> Option<String> {
    if proofs.is_empty() {
        return None;
    }
    let lines: Vec<String> = proofs
        .iter()
        .map(|p| format!("[{}] {}", p.id, p.references.join(", ")))
        .collect();
    Some(format!("Scripture References:\n{}", lines.join("\n")))
}

/// Escape HTML special characters.
pub fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

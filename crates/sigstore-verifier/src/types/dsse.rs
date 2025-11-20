use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    #[serde(rename = "_type")]
    pub statement_type: String,
    pub subject: Vec<Subject>,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub predicate: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    pub digest: HashMap<String, String>,
}

impl Statement {
    pub fn get_subject_digest(&self, algorithm: &str) -> Option<String> {
        self.subject
            .first()
            .and_then(|s| s.digest.get(algorithm).cloned())
    }
}

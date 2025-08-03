use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateEvent {
    ContentUpdate { 
        file_path: String, 
        html: String 
    },
    FileDeleted { 
        file_path: String 
    },
    Error { 
        message: String 
    },
}

impl UpdateEvent {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
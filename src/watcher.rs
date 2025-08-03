use crate::events::{FileEvent, UpdateEvent};
use crate::parser;
use crate::websocket::ConnectionManager;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{sleep, Instant};

pub struct FileWatcher {
    pub connection_manager: ConnectionManager,
    pub debounce_duration: Duration,
    watched_files: HashMap<PathBuf, FileMetadata>,
}

#[derive(Debug)]
struct FileMetadata {
    last_modified: Instant,
    content_hash: u64, // 可以用来检测内容是否真的改变了
}

impl FileWatcher {
    pub fn new(connection_manager: ConnectionManager) -> Self {
        Self {
            connection_manager,
            debounce_duration: Duration::from_millis(300),
            watched_files: HashMap::new(),
        }
    }

    pub async fn start_watching<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, mut rx) = mpsc::channel::<Result<Event, notify::Error>>(100);
        
        let config = Config::default()
            .with_poll_interval(Duration::from_millis(100));
        
        let mut watcher = RecommendedWatcher::new(
            move |result| {
                // Silently ignore channel send errors (happens when receiver is dropped)
                let _ = tx.blocking_send(result);
            },
            config,
        )?;
        
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
        
        println!("Started watching: {}", path.as_ref().display());
        
        while let Some(result) = rx.recv().await {
            match result {
                Ok(event) => self.handle_file_event(event).await,
                Err(e) => println!("File watch error: {}", e),
            }
        }
        
        // Keep the watcher alive
        std::mem::forget(watcher);
        Ok(())
    }

    async fn handle_file_event(&mut self, event: Event) {
        // Only process meaningful events for markdown files
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if self.is_markdown_file(&path) {
                        println!("Markdown file event: {:?} - {}", event.kind, path.display());
                        if self.should_process_file(&path).await {
                            self.process_markdown_file(&path).await;
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    if self.is_markdown_file(&path) {
                        println!("Markdown file deleted: {}", path.display());
                        let update_event = UpdateEvent::FileDeleted {
                            file_path: path.to_string_lossy().to_string(),
                        };
                        self.connection_manager.send_update(update_event).await;
                        self.watched_files.remove(&path);
                    }
                }
            }
            _ => {} // Ignore Access, Modify metadata, etc.
        }
    }

    async fn process_markdown_file(&mut self, path: &Path) {
        println!("Processing markdown file: {}", path.display());
        
        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                match parser::parse_markdown(&content) {
                    Ok(document) => {
                        let html = document.to_html();
                        let update_event = UpdateEvent::ContentUpdate {
                            file_path: path.to_string_lossy().to_string(),
                            html,
                        };
                        self.connection_manager.send_update(update_event).await;
                        
                        // Update file metadata
                        self.watched_files.insert(
                            path.to_path_buf(),
                            FileMetadata {
                                last_modified: Instant::now(),
                                content_hash: self.calculate_hash(&content),
                            },
                        );
                    }
                    Err(e) => {
                        let error_event = UpdateEvent::Error {
                            message: format!("Parse error in {}: {}", path.display(), e),
                        };
                        self.connection_manager.send_update(error_event).await;
                    }
                }
            }
            Err(e) => {
                println!("Failed to read file {}: {}", path.display(), e);
                let error_event = UpdateEvent::Error {
                    message: format!("Failed to read file {}: {}", path.display(), e),
                };
                self.connection_manager.send_update(error_event).await;
            }
        }
    }

    pub async fn watch_single_file<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = file_path.as_ref();
        if !path.exists() {
            return Err(format!("File does not exist: {}", path.display()).into());
        }
        
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            return Err(format!("Not a markdown file: {}", path.display()).into());
        }
        
        // Process the file once initially
        self.process_markdown_file(path).await;
        
        // Watch the parent directory to catch file modifications
        if let Some(parent) = path.parent() {
            self.start_watching(parent).await
        } else {
            Err("Cannot determine parent directory".into())
        }
    }

    fn is_markdown_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase() == "md")
            .unwrap_or(false)
    }

    async fn should_process_file(&mut self, path: &Path) -> bool {
        // Debouncing logic: check if enough time has passed since last processing
        if let Some(metadata) = self.watched_files.get(path) {
            let elapsed = metadata.last_modified.elapsed();
            if elapsed < self.debounce_duration {
                return false; // Too soon, skip this update
            }
        }
        true
    }

    fn calculate_hash(&self, content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}
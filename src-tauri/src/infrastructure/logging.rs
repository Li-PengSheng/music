use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_LINES: usize = 1000;

/// A small, shared log: every line goes to stderr, to a file in the app's data
/// folder, and to an in-memory buffer the UI can show.
pub struct LogBuffer {
    lines: Mutex<VecDeque<String>>,
    file: Mutex<Option<File>>,
}

impl LogBuffer {
    pub fn new(path: Option<PathBuf>) -> LogBuffer {
        let file = path.and_then(|p| {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            OpenOptions::new().create(true).append(true).open(p).ok()
        });
        LogBuffer {
            lines: Mutex::new(VecDeque::new()),
            file: Mutex::new(file),
        }
    }

    pub fn push(&self, message: impl AsRef<str>) {
        let line = format!("[{}] {}", timestamp(), message.as_ref());
        eprintln!("{line}");
        if let Ok(mut f) = self.file.lock() {
            if let Some(file) = f.as_mut() {
                let _ = writeln!(file, "{line}");
            }
        }
        if let Ok(mut lines) = self.lines.lock() {
            lines.push_back(line);
            while lines.len() > MAX_LINES {
                lines.pop_front();
            }
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines
            .lock()
            .map(|l| l.iter().cloned().collect())
            .unwrap_or_default()
    }
}

fn timestamp() -> String {
    // UTC wall clock, second resolution: enough to correlate a session.
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let day = secs % 86_400;
    format!("{:02}:{:02}:{:02}", day / 3600, (day % 3600) / 60, day % 60)
}

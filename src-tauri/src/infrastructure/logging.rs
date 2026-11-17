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
    /// Redact credentials before stderr, disk and the Console buffer, not only at IPC.
    pub fn push_redacted(&self, message: &str, secret: Option<&str>) {
        let safe = match secret.filter(|s| !s.is_empty()) {
            Some(secret) => message.replace(secret, "***"),
            None => message.to_string(),
        };
        self.push(safe);
    }
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

    /// Atomically read or clear the session buffer. The on-disk log is untouched.
    pub fn console_logs(&self, clear: bool) -> Result<Vec<String>, String> {
        let snapshot = {
            let mut lines = self
                .lines
                .lock()
                .map_err(|_| "运行日志不可用".to_string())?;
            if clear {
                lines.clear();
            }
            lines.iter().cloned().collect::<Vec<_>>()
        };
        Ok(snapshot.iter().map(|line| redact_paths(line)).collect())
    }
}

/// Strip absolute directory prefixes, including quoted paths containing spaces.
/// Applied at the IPC boundary so the original diagnostic file stays intact.
pub(crate) fn redact_paths(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < line.len() {
        let rest = &line[i..];
        let bytes = rest.as_bytes();
        let drive = bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && matches!(bytes[2], b'\\' | b'/');
        let previous = line[..i].chars().next_back();
        let boundary = previous.is_none_or(|c| c.is_whitespace() || "\"'=(「[:：".contains(c));
        let absolute = drive
            || rest.starts_with("\\\\")
            || (boundary
                && rest.starts_with('/')
                && bytes.get(1).is_some_and(|b| !b.is_ascii_whitespace()));
        if absolute {
            let closing_quote = match previous {
                Some('"') => Some('"'),
                Some('\'') => Some('\''),
                Some('「') => Some('」'),
                _ => None,
            };
            // Apostrophes are legal inside Windows names even when the log wraps
            // the whole path in single quotes. Use the closing quote of the field.
            let single_quoted = (closing_quote == Some('\'')).then(|| {
                let field_end = rest.find(['\n', '\r', '|']).unwrap_or(rest.len());
                rest[..field_end].rfind('\'').unwrap_or(field_end)
            });
            let end = single_quoted.unwrap_or_else(|| {
                rest.find(|c: char| {
                    if let Some(quote) = closing_quote {
                        c == quote || matches!(c, '\n' | '\r')
                    } else {
                        matches!(c, '"' | '\n' | '\r' | '|')
                    }
                })
                .unwrap_or(rest.len())
            });
            let path = &rest[..end];
            let leaf = path.trim_end().rsplit(['/', '\\']).next().unwrap_or("");
            out.push_str("<path>/");
            out.push_str(leaf);
            if path.ends_with(' ') {
                out.push(' ');
            }
            i += end;
        } else {
            let c = rest.chars().next().unwrap();
            out.push(c);
            i += c.len_utf8();
        }
    }
    out
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_are_redacted_before_disk_and_console_storage() {
        let path = std::env::temp_dir().join(format!("mm-token-log-{}.log", std::process::id()));
        let logs = LogBuffer::new(Some(path.clone()));
        logs.push_redacted(
            "[listenbrainz] Authorization: Token private-token HTTP_401",
            Some("private-token"),
        );
        let line = &logs.console_logs(false).unwrap()[0];
        assert!(line.contains("Token *** HTTP_401"));
        assert!(!line.contains("private-token"));
        assert!(!logs.lines()[0].contains("private-token"));
        drop(logs);
        assert!(!std::fs::read_to_string(&path)
            .unwrap()
            .contains("private-token"));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn paths_cover_windows_unc_posix_and_unicode_without_losing_context() {
        for path in [
            r"D:\私密 目录\歌.flac",
            "D:/Users/private/song.flac",
            r"\\server\private share\song.flac",
            "/home/private/music/song.flac",
        ] {
            let sanitized =
                redact_paths(&format!("[12:34:56] [audio] open \"{path}\" | rate=44100"));
            assert!(sanitized.starts_with("[12:34:56] [audio] open \"<path>/"));
            assert!(sanitized.ends_with("\" | rate=44100"));
            assert!(!sanitized.contains("private"));
            assert!(!sanitized.contains("私密"));
        }
        assert_eq!(
            redact_paths("[audio] open 'C:\\Users\\O'Brien\\Music\\song.flac' | rate=44100"),
            "[audio] open '<path>/song.flac' | rate=44100"
        );
        assert_eq!(
            redact_paths("[scan] visited=3 imported=2"),
            "[scan] visited=3 imported=2"
        );
        assert_eq!(
            redact_paths(r#"[audio] open "C:\Users\O'Brien\Music\song.flac" | rate=44100"#),
            "[audio] open \"<path>/song.flac\" | rate=44100"
        );
        assert_eq!(
            redact_paths("[scan] path：/home/private/song.flac | imported=1"),
            "[scan] path：<path>/song.flac | imported=1"
        );
        for path in [
            r"C:\Users\Private (Work)\Music\song.flac",
            r"C:\Users\Private [Work]，目录\Music\song.flac",
            r"C:\Users\O'Brien\Music\song.flac",
        ] {
            assert_eq!(
                redact_paths(&format!("[audio] open {path} | rate=44100")),
                "[audio] open <path>/song.flac | rate=44100"
            );
        }
    }

    #[test]
    fn console_buffer_remains_bounded() {
        let logs = LogBuffer::new(None);
        for i in 0..MAX_LINES + 2 {
            logs.push(format!("[test] {i}"));
        }
        let lines = logs.console_logs(false).unwrap();
        assert_eq!(lines.len(), MAX_LINES);
        assert!(lines[0].ends_with("[test] 2"));
    }

    #[test]
    fn console_reads_sanitizes_and_clears_only_memory() {
        let path = std::env::temp_dir().join(format!("mm-console-{}.log", std::process::id()));
        let logs = LogBuffer::new(Some(path.clone()));
        logs.push(r#"[audio] open "C:\Users\Private Name\Music\song.flac" 44100Hz"#);
        logs.push("[smtc] ready");
        let lines = logs.console_logs(false).unwrap();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("[audio] open"));
        assert!(lines[0].contains("song.flac"));
        assert!(!lines[0].contains("Private Name"));
        assert!(lines[0].contains("44100Hz"));
        assert_eq!(&lines[0][0..1], "[");
        assert_eq!(logs.console_logs(false).unwrap(), lines);
        assert!(logs.console_logs(true).unwrap().is_empty());
        assert!(logs.console_logs(false).unwrap().is_empty());
        logs.push("[scan] visited=3 imported=2");
        assert_eq!(logs.console_logs(false).unwrap().len(), 1);
        drop(logs);
        assert!(std::fs::read_to_string(&path)
            .unwrap()
            .contains("[smtc] ready"));
        std::fs::remove_file(path).unwrap();
    }
}

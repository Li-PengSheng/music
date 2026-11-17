//! Publish a prepared same-directory file atomically without overwriting a new destination.
use std::path::Path;

#[cfg(windows)]
pub fn publish_new(source: &Path, target: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_WRITE_THROUGH};
    let source: Vec<u16> = source.as_os_str().encode_wide().chain(Some(0)).collect();
    let target: Vec<u16> = target.as_os_str().encode_wide().chain(Some(0)).collect();
    // Both buffers are NUL terminated and live through the call. Omitting REPLACE_EXISTING
    // preserves a destination created by another process after the preflight.
    unsafe {
        MoveFileExW(
            PCWSTR(source.as_ptr()),
            PCWSTR(target.as_ptr()),
            MOVEFILE_WRITE_THROUGH,
        )
    }
    .map_err(|_| std::io::Error::last_os_error())
}
#[cfg(not(windows))]
pub fn publish_new(source: &Path, target: &Path) -> std::io::Result<()> {
    // Atomic no-clobber equivalent on the non-product test platform.
    std::fs::hard_link(source, target)?;
    let _ = std::fs::remove_file(source);
    Ok(())
}

use std::path::PathBuf;

/// Directs all tracing output to a file. Never stdout/stderr while the app
/// is live (A5). Returns the log file path.
pub fn init_tracing(log_dir: Option<PathBuf>) -> std::io::Result<PathBuf> {
    let dir = log_dir.unwrap_or_else(|| std::env::temp_dir().join("veloce"));
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("veloce.log");
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let writer = std::sync::Mutex::new(file);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .with_target(true)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    Ok(path)
}

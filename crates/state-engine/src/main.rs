use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let socket = runtime_dir.join("myos-state.sock");
    if socket.exists() {
        std::fs::remove_file(&socket)?;
    }

    let listener = UnixListener::bind(&socket)?;
    eprintln!("myos-state-engine listening on {}", socket.display());

    for mut stream in listener.incoming().flatten() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_secs();
        writeln!(stream, "{{\"status\":\"ok\",\"timestamp\":{timestamp}}}")?;
    }

    Ok(())
}

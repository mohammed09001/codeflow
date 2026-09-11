//! Local-only CodeFlow daemon lifecycle baseline.
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

#[derive(Default)]
struct DaemonState {
    projects: std::collections::BTreeMap<String, Vec<String>>,
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    listener.set_nonblocking(true)?;
    let running = Arc::new(AtomicBool::new(true));
    let mut state = DaemonState::default();
    let shutdown = running.clone();
    ctrlc_handler(shutdown);
    while running.load(Ordering::SeqCst) {
        let Ok((mut stream, _)) = listener.accept() else {
            thread::sleep(Duration::from_millis(10));
            continue;
        };
        let mut request = [0_u8; 1024];
        let size = stream.read(&mut request)?;
        if request[..size].starts_with(b"GET /v1/health") {
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}")?;
        } else if request[..size].starts_with(b"POST /v1/projects") {
            let project_id = format!("project-{}", state.projects.len() + 1);
            state.projects.insert(project_id.clone(), Vec::new());
            let body = format!("{{\"project_id\":\"{project_id}\"}}");
            let response = format!(
                "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            );
            stream.write_all(response.as_bytes())?;
        } else if request[..size].starts_with(b"POST /v1/projects/")
            && request[..size].windows(8).any(|part| part == b"/analyze")
        {
            let path = std::str::from_utf8(&request[..size]).unwrap_or_default();
            let project_id = path
                .split_whitespace()
                .next()
                .unwrap_or("/")
                .split('/')
                .nth(3)
                .unwrap_or("");
            if let Some(revisions) = state.projects.get_mut(project_id) {
                let revision = format!("r{}", revisions.len() + 1);
                revisions.push(revision.clone());
                let body =
                    format!("{{\"project_id\":\"{project_id}\",\"revision_id\":\"{revision}\"}}");
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(response.as_bytes())?;
            } else {
                stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")?;
            }
        } else {
            stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")?;
        }
    }
    Ok(())
}

#[cfg(windows)]
fn ctrlc_handler(_running: Arc<AtomicBool>) {}
#[cfg(not(windows))]
fn ctrlc_handler(_running: Arc<AtomicBool>) {}

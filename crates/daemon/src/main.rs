use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{fs, thread};

struct Session {
    key: [u8; 32],
    expires_at: Instant,
}

fn main() {
    let socket_path = "/tmp/cred_manager.sock";
    // Ensure clean start
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path).expect("Could not bind socket");

    let session_store: Arc<Mutex<Option<Session>>> = Arc::new(Mutex::new(None));

    println!("Daemon running on {}. Cache duration: 5 mins.", socket_path);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let store = Arc::clone(&session_store);
            thread::spawn(move || handle_client(stream, store));
        }
    }
}

fn handle_client(mut stream: UnixStream, store: Arc<Mutex<Option<Session>>>) {
    let mut buf = [0u8; 64];
    if let Ok(_size) = stream.read(&mut buf) {
        let mut guard = store.lock().unwrap();

        if buf.starts_with(b"SET ") {
            // Extract the 32 bytes following "SET "
            let mut key = [0u8; 32];
            key.copy_from_slice(&buf[4..36]);

            *guard = Some(Session {
                key,
                expires_at: Instant::now() + Duration::from_secs(300),
            });
            let _ = stream.write_all(b"OK");
        } else if buf.starts_with(b"GET") {
            if let Some(s) = &*guard {
                if Instant::now() < s.expires_at {
                    let _ = stream.write_all(&s.key);
                } else {
                    *guard = None; // Session expired
                }
            }
        }
    }
}

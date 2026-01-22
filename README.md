# vlt-rs

A terminal-based password manager written in Rust, designed for simplicity, security, and ease of use. 
It features a daemon mode that caches your master password for a configurable duration, allowing you to run commands without re-entering your password each time.

## Features

**Password Management** — Add, retrieve, and delete passwords through an intuitive terminal interface. All passwords are encrypted and stored securely in SQLite.

**Master Password Protection** — A single master password encrypts and protects all your stored credentials. You need to remember only one password to unlock everything.

**Daemon Mode** — Start the daemon to cache your master password for a configurable duration. Run commands without re-entering your password every time—perfect for safe, local environments where convenience matters.

**Lightweight & Terminal-Native** — No GUI bloat, no external dependencies. Manage your passwords directly from your shell.

## Usage

Unlock and manage passwords:
```bash
cargo run -p cli -- add          # Add a new password
cargo run -p cli -- get          # Retrieve a password
cargo run -p cli -- nuke         # Delete all passwords
```

Start the daemon to cache your master password:
```bash
cargo run -p daemon
```

Once the daemon is running, CLI commands will use the cached password, eliminating the need to enter it repeatedly.

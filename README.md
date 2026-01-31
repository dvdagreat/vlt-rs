## vlt-rs
A terminal-based, fully local password manager written in Rust. Secure, simple, and deliberate.

## Quick demo
<p align="center">
  <!-- <video src="./assets/demo.mp4" width="900" controls autoplay loop muted></video> -->
  <img src="./assets/demo.gif" width="1080" />
</p>

too fast? Here's the demo in video format: <a href="./assets/demo.mp4"> Video Link </a>

## Installation & Setup

### 1. Clone the Repository
```bash
git clone https://github.com/dvdagreat/vlt-rs.git
cd vlt-rs
```

### 2. Compile the Project
```bash
# Build the main CLI
cargo build --release --bin vlt

# Build the daemon (optional)
cargo build --release --bin vlt_daemon
```

if you see an error like like 
```bash
= note: some arguments are omitted. use `--verbose` to show all linker arguments
= note: rust-lld: error: unable to find library -lsqlite3
        collect2: error: ld returned 1 exit status
```

You probably need libsqlite3 library install. Install it via below command
```bash 
sudo apt update

sudo apt install libsqlite3-dev
```

### 3. Configure Aliases
Add these to your .bashrc or .zshrc for easy access (replace [path-to-repo] with your actual directory):
```bash
alias vlt="[path-to-repo]/target/release/vlt"
alias vlt_daemon="[path-to-repo]/target/release/vlt_daemon"
```

---

## Usage
### Adding a credential:
``` bash
vlt pass add
```

### Retrieving a credential to clipboard:
```bash
vlt pass get
```

### Viewing help and commands:
```bash
vlt --help
# or for specific commands
vlt pass --help
```

## Using the Daemon
If you don't want to type your master password for every command, run the daemon in a dedicated terminal window:
```bash
vlt_daemon
```

While running, vlt will automatically pull the cached password for 5 minutes.

---

## Goals
`vlt` is designed to be a universal store for:

Credentials and passwords

Access tokens (API keys, OAuth tokens, etc.)

Security and emergency recovery keys

Private keys (SSH, etc.)

Essentially, if it can be stored as text, `vlt` can protect it behind a single master password.


## Why another password manager?
Most modern solutions (1Password, NordPass, Proton Pass, etc.) are cloud-based and closed-source. While convenient, they introduce specific risks that `vlt` eliminates:

Cloud Exposure: Cloud-based credentials can technically be accessed from anywhere—even if you don't want them to be.

Single Point of Failure: Large-scale password SaaS platforms are "honeypots" for hackers; a single breach can expose millions of users.

Closed Source Lack of Transparency: Proprietary encryption logic lacks public auditability. You have to trust the company is using industry-grade methods.

True Zero-Knowledge: Many platforms claim zero-knowledge, but they still manage your recovery and account access.


## The `vlt` Approach
Local-First: `vlt` is fully offline. Your data never leaves your machine. No "Account Compromised" emails, because there is no account—just your local system.

Open Source: The code is transparent and open for community audit. Vulnerabilities can be identified and patched by anyone, free from corporate gatekeeping.

Strict Zero-Knowledge: Security is enforced locally. If you lose your master password, even the tool cannot recover your data. You are in total control.

## Features
Secure Storage — Add, retrieve, and manage credentials via an intuitive CLI. Data is stored in a locally encrypted SQLite database.

Master Password Protection — Only one password to remember. The master password is never stored on disk, placing the full weight of security in the user's hands.

Smart Daemon Tool — The vlt_daemon caches your master password in memory for 5 minutes, balancing high security with developer productivity.

Lightweight & Native — No GUI bloat or heavy external dependencies. Built for speed and shell integration.

Hardened Security — Utilizes the Argon2 hashing algorithm for robust key derivation, ensuring your data is protected by industry-standard cryptography.

## Downsides of using `vlt`
- (accessibility) You lose the ability to access your passwords across machines.

- (accessibility) You can't use Autofill (...yet! I have another tool planned for this purpose)

- (human psychology) You need to be careful against social engineering by hackers. If they get your master password then they can access all your passwords. 
Although `groups` feature (still WIP), can help reduce the risk of full exposure of credentials but partial exposure risk still exists

- (accessibility) If you forget your master password, there is no way to restore your password due to zero knowledge architecture


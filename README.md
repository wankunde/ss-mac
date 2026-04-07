# ss-mac 🚀

A macOS-native implementation of the Linux `ss` (socket statistics) tool, written in Rust. Since macOS lacks the Netlink API, this tool uses Darwin-specific `sysctl`, `libproc`, and `lsof` to provide an identical interface and output format as the standard Ubuntu/Linux `ss` utility.

## ✨ Features

- **Protocols**: TCP (`-t`), UDP (`-u`), and UNIX Domain Sockets (`-x`).
- **States**: Filter by Listening (`-l`) or All (`-a`) sockets.
- **Process Mapping**: Show which process owns each socket with the `-p` flag (mimics Linux output).
- **Numeric Output**: Skip hostname and service resolution with the `-n` flag.
- **Cross-Architecture**: Supports both Intel (x86_64) and Apple Silicon (M1/M2/M3) Macs.

## 🛠 Usage

```bash
# Display all listening TCP sockets with process names
ss-mac -tlp

# Display all TCP and UDP connections (listening and established)
ss-mac -atu

# Display all UNIX domain sockets
ss-mac -x

# Display help and all available options
ss-mac --help
```

## 📦 Installation

### One-Line Install (Recommended)
You can install the pre-built binary directly from GitHub Releases using this one-line command. You don't need to have Rust installed:

```bash
curl -fsSL https://raw.githubusercontent.com/wankunde/ss-mac/main/install.sh | bash
```

### Build from Source
If you have Rust and Cargo installed and prefer to build from source:
```bash
git clone https://github.com/wankunde/ss-mac.git
cd ss-mac
cargo install --path .
```

## 🔨 Development

### Build
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

## 🏗 GitHub Actions
The project includes a CI workflow that automatically runs:
1. **Linting**: Checks code formatting and common errors.
2. **Testing**: Runs the integrated unit tests.
3. **Build**: Ensures the project compiles on macOS environments.

## ⚖️ License
MIT / Apache 2.0

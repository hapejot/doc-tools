# Rust-based Rsync Solutions for Windows

## Question

Is there a Rust solution for an rsync agent that is compatible with a Linux rsync implementation?
I need one for Windows but I don't want to implement a full Linux environment just for one program.

## Answer

**Yes!** There is a Rust-based rsync implementation that is wire-protocol compatible with Linux rsync and works on Windows.

## Recommended Solution: oc-rsync

**[oc-rsync](https://github.com/oferchen/rsync)** (oferchen/rsync) is a classic rsync re-implementation in pure Rust that is designed to be wire-protocol compatible with the original rsync.

### Key Features

- **Protocol Compatible**: Implements rsync protocol 32 (the current/latest version as of 2025), making it fully compatible with standard Linux rsync installations
- **Cross-platform**: Works on Windows, Linux, and macOS
- **Drop-in Replacement**: Can act as both client and daemon, interoperating with Linux rsync servers
- **Pure Rust**: Written in Rust for safety, security, and performance
- **No Linux Environment Required**: Runs natively on Windows without WSL, Cygwin, or other emulation layers

### Installation

#### From Crates.io
```bash
cargo install oc-rsync
```

#### From Source
```bash
git clone https://github.com/oferchen/rsync
cd rsync
cargo build --release
```

The binary will be in `target/release/oc-rsync` (or `oc-rsync.exe` on Windows).

### Usage

oc-rsync aims to replicate the standard rsync command-line interface:

```bash
# Basic file sync from Windows to Linux server
oc-rsync -avz /path/to/local/folder user@linux-server:/path/to/remote/folder

# Sync from Linux server to Windows
oc-rsync -avz user@linux-server:/path/to/remote/folder C:\local\folder

# Local sync on Windows
oc-rsync -av C:\source\folder C:\destination\folder

# Dry run to see what would be synced
oc-rsync -avn source/ destination/
```

Common options (similar to rsync):
- `-a` : Archive mode (recursive, preserve permissions, times, etc.)
- `-v` : Verbose output
- `-z` : Compress data during transfer
- `-n` : Dry run (don't actually transfer files)
- `--delete` : Delete files in destination that don't exist in source

## Alternative Solutions

### 1. rusync

**[rusync](https://github.com/your-tools/rusync)** is another Rust-based file synchronization tool.

**Important Limitation**: rusync is **NOT** wire-protocol compatible with the standard rsync protocol. It cannot communicate with Linux rsync servers over the network.

**Best For**: Local folder synchronization on Windows or between local drives.

```bash
cargo install rusync
rusync source/ destination/
```

### 2. WSL (Windows Subsystem for Linux)

If you need 100% compatibility with Linux rsync, you can use WSL.

**Pros**:
- Runs the actual Linux rsync binary
- Perfect compatibility

**Cons**:
- Requires installing WSL
- Adds system overhead
- More complex setup

```bash
# Install WSL, then:
wsl sudo apt update
wsl sudo apt install rsync
wsl rsync -avz source/ destination/
```

### 3. Cygwin or MSYS2

These provide ports of rsync compiled for Windows.

**Pros**:
- Mature, well-tested implementations
- Good compatibility

**Cons**:
- Requires installing Cygwin or MSYS2 environment
- Larger installation footprint than a single binary

## Comparison

| Solution | Protocol Compatible | Native Windows | Lightweight | Pure Rust |
|----------|-------------------|----------------|-------------|-----------|
| **oc-rsync** | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
| rusync | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes |
| WSL | ✅ Yes | ⚠️ Via WSL | ❌ No | ❌ No |
| Cygwin/MSYS2 | ✅ Yes | ⚠️ Via Emulation | ❌ No | ❌ No |

## Recommendation

For your use case (rsync on Windows without a full Linux environment and with Linux rsync compatibility), **oc-rsync is the ideal solution**.

It provides:
1. ✅ Native Windows support (no Linux environment needed)
2. ✅ Wire-protocol compatibility with Linux rsync
3. ✅ Written in Rust (safe, fast, and modern)
4. ✅ Single binary deployment
5. ✅ Active development and maintenance

## Additional Resources

- [oc-rsync GitHub Repository](https://github.com/oferchen/rsync)
- [rusync GitHub Repository](https://github.com/your-tools/rusync)
- [Original rsync documentation](https://rsync.samba.org/)
- [rsync protocol specification](https://rsync.samba.org/tech_report/)

## Contributing

If you find issues or have improvements for this documentation, please open an issue or pull request in this repository.

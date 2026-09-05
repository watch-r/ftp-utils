# mntisp – Mount ISP Media via rclone

## Overview

`mntisp` is a tiny Rust command‑line utility that helps me (and anyone with a compatible ISP) mount an **FTP over HTTPS** media share directly into the file manager (Dolphin).  It wraps `rclone mount` and provides a simple interface to start, stop, check the status, and view a short activity log.

## Why this project exists

I wanted to access my ISP‑provided media library (music, video, etc.) from the **Dolphin** file manager as if it were a local folder. The ISP only exposes the media share through a proprietary HTTPS FTP endpoint, which isn’t natively mountable. By delegating the heavy lifting to `rclone` and handling the mount lifecycle in Rust, I get a convenient, repeatable way to bring that remote folder into my `$HOME/Downloads/ISP_Media` directory.

## Supported ISP

The current implementation works **only with a specific ISP** that serves the media share at:

```
https://172.16.16.6
```

If your ISP uses a different URL, authentication method, or mount options, you’ll need to edit the source code (see *Configuration* below).

## Features

- `mntisp start` – creates the mount point (if needed) and launches `rclone mount` in daemon mode.
- `mntisp stop` – cleanly unmounts the share using `fusermount3` (with a lazy fallback).
- `mntisp status` – reports whether the mount point is currently active and displays basic disk usage.
- `mntisp log` – shows the last 15 lines of activity logged to `~/.mntisp.log`.

All actions are logged with timestamps for troubleshooting.

## Configuration

The command‑line arguments for `rclone` are hard‑coded in **`src/mount.rs`**:

```rust
let spawn_result = Command::new("rclone")
    .args([
        "mount",
        ":http:",
        target_str,
        "--http-url",
        "https://172.16.16.6",
        "--no-check-certificate",
        "--read-only",
        "--vfs-cache-mode",
        "minimal",
        "--daemon",
    ])
    .status();
```

To adapt the tool for a different ISP:
1. Open `src/mount.rs`.
2. Replace the URL (`https://172.16.16.6`) with your provider’s endpoint.
3. Adjust any additional `rclone` flags (e.g., authentication, cache mode) as required.
4. Re‑compile with `cargo build --release`.

## Installation & Usage

```bash
# Clone the repository
git clone <repo‑url>
cd load_ftp

# Build (requires Rust toolchain)
cargo build --release

# Run the binary (the compiled binary is at target/release/mntisp)
./target/release/mntisp start   # mount the share
./target/release/mntisp status  # check mount status
./target/release/mntisp log     # view recent logs
./target/release/mntisp stop   # unmount
```

You can create a desktop entry or a custom Dolphin service to invoke the binary for a one‑click mount experience.

## Limitations & Known Issues

- Only works on Linux systems where `fusermount3` and `rclone` are available.
- The mount point is fixed to `~/Downloads/ISP_Media`.
- No automatic ISP detection – you must manually edit the source code for other providers.
- The tool assumes the remote server uses a self‑signed certificate (`--no-check-certificate`). Adjust the flag if your server uses a valid cert.

## Contributing

If you encounter a different ISP that can be supported with a small change, feel free to fork the repo, update the URL/flags, and submit a pull request.

---

*This project is a personal utility; it is not an official client from the ISP.*

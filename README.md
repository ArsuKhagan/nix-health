# nix-health

`nix-health` is a minimal, read-only CLI inspector designed for NixOS environments. It provides quick diagnostics regarding configuration health and `/nix/store` disk footprint.

## Features

- **Broken Symlink Audit:** Recursively scans `~/.config` for invalid symbolic links while filtering out socket lock files and ephemeral node/npm artifacts.
- **Store Analytics:** Queries the Nix daemon (`nix path-info`) to report registered store paths and aggregate space usage in gigabytes.
- **Read-Only Operation:** Performs purely non-destructive checks without altering system state.

## Output Overview

:: nix-health v0.1.0 — NixOS Environment Inspector
─────────────────────────────────────────────────────────────────

[1/2] Scanning configurations in ~/.config...
  SUCCESS: All symlinks are valid. No broken references found.

[2/2] Analyzing /nix/store state...
  • Registered store paths: 14820
  • Total store size:        24.50 GB

 Audit completed successfully. System state unchanged (Read-Only).

## Installation & Build

### Requirements

- Rust toolchain (`cargo`, `rustc`)
- Nix / NixOS system environment

### Building from Source

git clone [https://github.com/ArsuKhagan/nix-health.git](https://github.com/ArsuKhagan/nix-health.git)
cd nix-health
cargo build --release

The compiled binary will be available at `./target/release/nix-health`.

## License

MIT

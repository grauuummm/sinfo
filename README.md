# sinfo - A Minimal System Information Tool

sinfo is a simple command-line application written in Rust that displays system information in a clean and minimalistic format, similar to Neofetch. It shows essential system details such as the OS name, kernel version, desktop environment or window manager, RAM usage, package count, and CPU information.

## Features

- Gathers and displays basic system information:
  - OS Name
  - Kernel Version
  - Desktop Environment or Window Manager
  - RAM Usage (used/total, in GB)
  - Package Count (for supported package managers)
  - CPU Model and Clock Speed

## Installation

To build and install sinfo system-wide, run:

```bash
./install.sh
```

This will compile sinfo and install it to `/usr/local/bin` (you may be prompted for your password).

Alternatively, you can build and run manually:

```bash
cargo build --release
./target/release/sinfo
```

## Usage

After building or installing, you can run the application with:

```bash
sinfo
```
or, if not installed system-wide:
```bash
./target/release/sinfo
```

## Supported Package Managers

- `dpkg` (Debian/Ubuntu)
- `pacman` (Arch Linux)
- `rpm` (Fedora/openSUSE)

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

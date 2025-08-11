# sinfo

A minimal system information display tool written in Rust

## Example Output
```
distro: debian gnu/linux
krnl: 6.4.0-1-amd64
de: gnome
ram: 2.34 gb / 7.67 gb
pkgs: 2137
cpu: amd ryzen 5 3500u @ 2.10 ghz
init: systemd
```

## Features

- Displays system information in a minimal format:
  - Distribution name
  - Kernel version
  - Desktop environment
  - RAM usage (used/total)
  - Package count
  - CPU model and clock speed
  - Init
- No external dependencies
- Fast execution

## Installation

### From Source
1. Clone the repository:
```bash
git clone https://github.com/grauuummm/sinfo.git
cd sinfo
```

2. Install system-wide (requires sudo):
```bash
./install.sh
```

Or build manually:
```bash
cargo build --release
./target/release/sinfo
```

### Requirements
- Rust/Cargo
- Linux distro

## Usage

If installed system-wide:
```bash
sinfo
```

If built manually:
```bash
./target/release/sinfo
```

## Supported Systems
every distro that has its name written in /etc/os-release

### Package Managers
- `dpkg` 
- `pacman` 
- `rpm` 

### Init Systems
- automatically detected

## Future Plans
- [ ] ASCII art logos for distributions
- [ ] More package manager support
- [ ] Configurable output format
- [ ] Color output support

## Contributing

Contributions are welcome! Feel free to:
- Open an issue for bugs or feature requests
- Submit a pull request
- Suggest improvements

## License

This project is open source. Feel free to use, modify, and distribute as you wish.

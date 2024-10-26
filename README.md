# laptop-support-cli

Command line tool for laptop-support protocol.


## Features

Control your laptop with simple commands via laptop-support protocol.

- Tablet mode
  - Enable / Disable
  - Enable / Disable auto-switching
  - Get status
- Keyboard
  - Enable / Disable
  - Get status
- Mouse
  - Enable / Disable
  - Get status

## Requirements

Pre-installed `laptop-support` compatible service in your system.

- [minibook-support](https://github.com/petitstrawberry/minibook-support) for CHUWI MiniBook and MiniBook X

## Installation

### From AUR

Recommended way for ArchLinux users.

```sh
yay -S laptop-support-cli-git
```

### From Source

#### Prerequisites

- Rust
- Cargo

#### Build and Install

```sh
git clone https://github.com/petitstrawberry/laptop-support-cli.git
cd laptop-support-cli
cargo install --path .
```

## Usage

```sh
laptopcli --help
```

### Example

Enable tablet mode.

```sh
sudo laptopcli tabletmode enable
```

## License

MIT License

Copyright (c) 2024 petitstrawberry

Please see [LICENSE](LICENSE) for more information.

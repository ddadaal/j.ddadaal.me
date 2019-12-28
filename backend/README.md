# Backend for j.ddadaal.me

Backend for j.ddadaal.me written in Rust with [actix-web](https://github.com/actix/actix-web).

## Features

- RESTful HTTP API
- Async for all its glory!

## Development 

### Dev Environment

- Arch Linux WSL
- Rust under WSL
- VSCode Remote Development connecting to WSL
- docker & docker-compose 

## Getting Started

1. Make sure Rust is ready.
2. Install `SQLite`

```bash
sudo pacman -S sqlite
```

3. Install [`diesel_cli`](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with only `sqlite` support
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

4. Run migration
```bash
diesel migration run
```

## Misc 

### Useful tools

- [cargo-edit](https://github.com/killercup/cargo-edit)
  - Add `add`, `rm`, `upgrade` to `cargo` to manage dependencies
  - `cargo install cargo-edit`
- rust-clippy
  - Code analyzer to catch common mistakes, `clippy`
  - `rustup component add clippy`
- [diesel_cli_ext](https://github.com/abbychau/diesel_cli_ext)
  - Auto generate struct based on diesel schema
  - `cargo install diesel_cli_ext`

### Install docker under Arch WSL and connect it to Docker on Windows

```bash
# Install docker and docker-compose
sudo pacman -S docker docker-compose

# Setup docker host
echo "export DOCKER_HOST=tcp://localhost:2375" >> ~/.zshrc && source ~/.zshrc
```
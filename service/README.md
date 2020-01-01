# Service for j.ddadaal.me

Service for j.ddadaal.me written in Rust with [actix-web](https://github.com/actix/actix-web).

## Features

- RESTful HTTP API
- Async for all its glory!

## API Docs

Only one API is implemented:

```
Request: GET /{short_link}

Response:
- 301, header Location is set to the target URL
- 404, no target URL associated to the provided short_link is found
- 500, Internal server error

```

## Development 

### Dev Environment

- Arch Linux WSL
- Rust under WSL
- VSCode Remote Development connecting to WSL

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
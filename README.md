# rust-url-shortener

Yet another URL shortener written in Rust with [actix-web](https://github.com/actix/actix-web).

## Features

- [actix-web](https://github.com/actix/actix-web) for the best performance possible
- Async for all its glory!

## API Docs

```
Request: GET /{short_link}

Response:
- 301, header Location is set to the target URL
- 404, no target URL associated to the provided short_link is found
- 500, Internal server error

```

## Run & Dev

It is minimually usable now, but the target URLs must be set **manually** to the production database (Only SQLite is supported for now.)

### Environment

Only Linux is tested. The development is under

- Arch Linux WSL
- Rust under WSL
- VSCode Remote Development connecting to WSL
- VSCode Rust extension with RLS

### Getting Started

1. Make sure the latest Rust (>=1.40) is installed properly.
2. Install `SQLite`

```bash
sudo pacman -S sqlite
```

3. Install [`diesel_cli`](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) with only `sqlite` support
```bash
cargo install diesel_cli --no-default-features --features sqlite
```

4. Run migration to initialize database
```bash
diesel migration run
```

5. Set some jump rules to the `prod.db` file under the root directory of service. The schema can be viewed through `/migrations` folder or sqlite3 CLI.

```bash
# open the prod.db with sqlite3 cli
sqlite3 prod.db

# for all tables
> .tables

# to view the schema of a table
> .schema {table_name}
```

6. Run

```bash
cargo run
```

7. The server is started at http://localhost:8080.

### Docker

Build image:

```bash
docker built -t myapp:latest . 
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
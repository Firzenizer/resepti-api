# resepti-api

## Technologies
- Rust
- Framework [Actix](https://actix.rs/)
- ORM [SeaORM](https://github.com/SeaQL/sea-orm)

## Prerequisites
- Install rust using [rustup](https://rustup.rs/)
- Install [Docker](https://docs.docker.com/get-docker/)

## Nice to have tools
- [DBeaver](https://dbeaver.io/download/) - Interacting with DB

## Installation
- Run:
  - `docker compose up --build --detach` # Creates Docker containers and starts them
  - `cargo install cargo-watch` # Installs watcher for autobuilding
  - `cargo install sea-orm-cli` # Installs SeaORM CLI for migrations
  - `cargo install --path ./`
  - `sea-orm-cli migrate up` # Runs migrations
  - `cd api` & run `cargo watch -x run`

## Useful commands
- Start server and watcher: `cargo watch -x run`
- Generate entities from migration: `sea-orm-cli generate entity -o entity/src`
# anthill

- server is running on `http://localhost:8080`
- connects to postgres database as configured in `.env`

```bash
cargo build
cargo run
# RUST_LOG=DEBUG cargo run
# $env:RUST_LOG="DEBUG"; cargo run

# or for UI development
cargo run -- --port 8081
cd web
yarn serve
```

## Building on Windows
- [Install `PostgreSQL`](https://www.enterprisedb.com/downloads/postgres-postgresql-downloads) for the libpq database driver build/runtime dependency
- Grab a new [libintl-9.dll](https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857) to fix a crash upon connecting to postgres.

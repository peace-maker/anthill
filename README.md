# anthill

- server is running on `http://localhost:8080`
- connects to postgres database as configured in `.env`

```bash
cargo build
cargo run
# RUST_LOG=DEBUG cargo run

# or for UI development
cargo run -- --port 8081
cd web
yarn serve
```
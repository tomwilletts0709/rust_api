# Rust API – Book Store

A REST-style API built with **Rocket**, **Diesel**, and **PostgreSQL**. Serves static files from `public/` and uses r2d2 for database connection pooling.

## Prerequisites

- [Rust](https://rustup.rs) (nightly required for Rocket 0.4: `rustup default nightly`)
- [PostgreSQL](https://www.postgresql.org/) or [Postgres.app](https://postgresapp.com/) on macOS
- [Diesel CLI](https://diesel.rs/guides/getting-started/) for migrations:  
  `cargo install diesel_cli --no-default-features --features postgres`

## Quick Start

```bash
git clone https://github.com/tomwilletts0709/rust_api.git
cd rust_api
```

1. Create `.env` in the project root:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost/rust_api_book_store
   ```

2. **Postgres.app on macOS** – set the library path:
   ```bash
   export LIBRARY_PATH="/Applications/Postgres.app/Contents/Versions/18/lib:$LIBRARY_PATH"
   ```
   Add to `~/.zshrc` for persistence.

3. Create database and run migrations:
   ```bash
   cd api
   diesel database setup
   ```

4. Run the server:
   ```bash
   cargo run
   ```

Server runs at **http://localhost:8000**. Serves `public/index.html` at `/`.

> Rocket 0.4 requires Rust nightly. If the build fails with a Pear/nightly error, run `rustup default nightly`.

## Project Structure

```
rust_api/
├── api/
│   ├── src/
│   │   ├── main.rs         # Rocket entry, pool setup
│   │   ├── models.rs       # Book model and CRUD
│   │   ├── schema.rs       # Diesel schema (generated)
│   │   ├── db.rs           # r2d2 pool + Conn request guard
│   │   └── static_files.rs # Static file routes
│   ├── public/             # Static assets
│   ├── migrations/         # SQL migrations
│   └── Cargo.toml
├── .env                    # DATABASE_URL (gitignored)
└── README.md
```

## Dependencies

| Crate     | Purpose                      |
| --------- | ---------------------------- |
| Rocket    | Web framework                |
| Diesel    | PostgreSQL ORM               |
| r2d2      | Connection pooling           |
| dotenv    | `.env` loading                |
| serde     | Serialization                |

## License

MIT

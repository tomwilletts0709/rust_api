# Rust API – Book Store

A Rust API using [Rocket](https://rocket.rs), [Diesel](https://diesel.rs), and PostgreSQL for a simple book store backend. Serves static files from `public/` and uses r2d2 for connection pooling.

## Prerequisites

- [Rust](https://rustup.rs) (Rocket 0.4 requires nightly: `rustup default nightly`)
- [PostgreSQL](https://www.postgresql.org/) (or [Postgres.app](https://postgresapp.com/) on macOS)

## Setup

1. **Clone the repo**
   ```bash
   git clone https://github.com/tomwilletts0709/rust_api.git
   cd rust_api
   ```

2. **Create a `.env` file** in the project root:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost/rust_api_book_store
   ```
   Adjust the connection string for your PostgreSQL setup.

3. **Postgres.app on macOS** – If you use Postgres.app, set the library path so Diesel can link:
   ```bash
   export LIBRARY_PATH="/Applications/Postgres.app/Contents/Versions/18/lib:$LIBRARY_PATH"
   ```
   Add this to your `~/.zshrc` to make it persistent.

4. **Create the database and run migrations**
   ```bash
   cd api
   diesel database setup
   ```

## Run

```bash
cd api
cargo run
```

Server runs at `http://localhost:8000` by default. Serves `public/index.html` at `/`.

> **Note:** Rocket 0.4 uses proc macros that require Rust nightly. If the build fails with a Pear/nightly error, run `rustup default nightly` first.

## Project Structure

```
rust_api/
├── api/
│   ├── src/
│   │   ├── main.rs        # Rocket entry point
│   │   ├── models.rs      # Book model and CRUD operations
│   │   ├── schema.rs      # Diesel schema (auto-generated)
│   │   ├── db.rs          # r2d2 connection pool and Conn guard
│   │   └── static_files.rs # Static file routes
│   ├── public/            # Static assets (index.html, etc.)
│   ├── migrations/       # SQL migrations
│   └── Cargo.toml
├── .env                   # DATABASE_URL (not committed)
└── README.md
```

## License

MIT

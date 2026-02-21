# Rust API – Book Store

A Rust API using [Diesel](https://diesel.rs) and PostgreSQL for a simple book store backend.

## Prerequisites

- [Rust](https://rustup.rs)
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

## Project Structure

```
rust_api/
├── api/
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── models.rs    # Book model and CRUD operations
│   │   └── schema.rs    # Diesel schema (auto-generated)
│   ├── migrations/     # SQL migrations
│   └── Cargo.toml
├── .env                 # DATABASE_URL (not committed)
└── README.md
```

## License

MIT

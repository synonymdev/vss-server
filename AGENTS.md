# WARP.md

This file provides guidance to AI agents like Claude Code/WARP/Cursor/Codex when working with code in this repository.

## Project Overview

VSS (Versioned Storage Service) is a server-side cloud storage solution for non-custodial Lightning Network wallets. It enables wallet state recovery and multi-device access. The project has two implementations: a production-ready Java version and a work-in-progress Rust version.

## Build and Run Commands

### Java Implementation (`java/`)

```bash
# Setup (first time)
gradle wrapper --gradle-version 8.1.1

# Build (skip tests if Docker not available)
./gradlew build -x test

# Build with tests (requires Docker for PostgreSQL testcontainers)
./gradlew build

# Run with PostgreSQL container
docker-compose up --build

# Generate protobuf objects (after proto changes)
./gradlew generateProto

# Generate jOOQ objects (after schema changes)
./gradlew generateJooq
```

### Rust Implementation (`rust/`)

```bash
# Build
cargo build --release

# Run (requires PostgreSQL and config file)
cargo run -- server/vss-server-config.toml

# Run tests
cargo test --workspace
```

## Architecture

### API Contract

The API is defined in `proto/vss.proto` using Protocol Buffers. Both implementations must adhere to this contract. The API exposes four operations via HTTP POST endpoints at `/vss/`:

- `getObject` - Retrieve a key-value pair
- `putObjects` - Store/update key-value pairs (supports transactions)
- `deleteObject` - Delete a key-value pair
- `listKeyVersions` - List keys with their versions (paginated)

### Core Interface

Both implementations define a `KvStore` trait/interface:
- Java: `java/app/src/main/java/org/vss/KVStore.java`
- Rust: `rust/api/src/kv_store.rs`

### Rust Workspace Structure

- `api/` - Core types, traits (`KvStore`, `Authorizer`), error types, and test suite
- `impls/` - Backend implementations (PostgreSQL)
- `auth-impls/` - Authorization implementations (JWT)
- `server/` - HTTP server using hyper

### Java Package Structure

- `org.vss.api` - REST endpoints extending `AbstractVssApi`
- `org.vss.impl.postgres` - PostgreSQL backend using jOOQ
- `org.vss.auth` - Authorization (`JwtAuthorizer`, `NoopAuthorizer`)
- `org.vss.exception` - Custom exceptions (`ConflictException`, `AuthException`, `NoSuchKeyException`)

### Database Schema

Single table `vss_db` with composite primary key `(user_token, store_id, key)`. Schema files:
- Java: `java/app/src/main/java/org/vss/impl/postgres/sql/v0_create_vss_db.sql`
- Rust: `rust/impls/src/postgres/sql/v0_create_vss_db.sql`

## Configuration

### Java

Config via `java/app/src/main/resources/application.properties`. All properties can be overridden by environment variables with the same name (e.g., `vss.jdbc.url`).

### Rust

Config via TOML file (default: `rust/server/vss-server-config.toml`). All values can be overridden by environment variables prefixed with `VSS_` (e.g., `VSS_POSTGRESQL_HOST`).

## Versioning

VSS supports two levels of versioning:
1. **Key-level versioning**: Optimistic concurrency control per key. Use version `-1` to skip version checks.
2. **Global versioning**: Store-wide sequence number for multi-device synchronization.

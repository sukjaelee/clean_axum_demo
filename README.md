# Rust Axum Clean Demo

A modern, clean-architecture Rust API server template built with Axum and SQLx. It incorporates domain-driven design, repository patterns, JWT authentication, file uploads, Swagger documentation, and comprehensive testing.

---

## ✨ Features

- Clean architecture with layered domain separation
- Modular Axum HTTP server with route handlers
- SQLx with compile-time checked queries
- JWT authentication and protected routes
- Asynchronous file upload and secure asset serving
- Swagger UI documentation powered by Utoipa
- OpenTelemetry distributed tracing and metrics instrumentation

---

## 📦 Project Structure

Recommended layout:

```
├── src
│   ├── <domain>/             # e.g., auth, user, device, file
│   │   ├── mod.rs            # Module entry point
│   │   ├── domain/           # Domain logic: models, traits
│   │   │   ├── mod.rs
│   │   │   ├── model.rs
│   │   │   ├── repository.rs
│   │   │   └── service.rs
│   │   ├── handlers.rs       # Route handlers
│   │   ├── routes.rs         # Route definitions
│   │   ├── queries.rs        # SQLx query logic
│   │   ├── dto.rs            # Data Transfer Objects
│   │   └── services.rs       # Infrastructure-layer service implementations
│
│   ├── common/               # Shared components and utilities
│   │   ├── mod.rs
│   │   ├── app_state.rs      # AppState struct for dependency injection
│   │   ├── bootstrap.rs      # Service initialization and AppState construction
│   │   ├── config.rs         # Environment variable configuration loader
│   │   ├── dto.rs            # Shared/global DTOs
│   │   ├── error.rs          # AppError enum and error mappers
│   │   ├── hash_util.rs      # Hashing utilities (e.g., bcrypt)
│   │   ├── jwt.rs            # JWT encoding, decoding, and validation
│   │   ├── opentelemetry.rs  # OpenTelemetry setup
│   │   └── ts_format.rs      # Custom timestamp serialization formatting
│
│   ├── lib.rs                # Declares top-level modules like app, auth, user, etc.
│   ├── app.rs                # Axum router and middleware setup
│   ├── main.rs               # Application entry point
│
├── db-seed/                  # Database table definitions and seed data
├── tests/                    # Integration and API tests
│   ├── asset/                # Test file assets
│   ├── test_auth_routes.rs
│   ├── test_device_routes.rs
│   ├── test_helpers.rs       # Shared setup and utilities for tests
│   └── test_user_routes.rs
├── .env                     # Environment variables for local development
├── .env.test                # Environment overrides for test environment
└── ERD.png                  # Database Entity Relationship Diagram
```

> When adding a new domain module, register it in:
>
> - `src/lib.rs`
> - `src/app.rs`
> - `src/common/app_state.rs`
> - `src/common/bootstrap.rs`

---

## 🛠 Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Docker & Docker Compose (optional)
- For MySQL version, see [clean_axum_demo_mysql](https://github.com/sukjaelee/clean_axum_demo_mysql)

### Quickstart

Choose your preferred setup:

**Using Docker Compose:**

```bash
docker-compose up --build
```

To stop and clean up:

```bash
docker-compose down --rmi all
```

**Manual Setup:**

1. Create database tables and seed data:

   ```bash
   db-seed/01-tables.sql
   db-seed/02-seed.sql
   ```

2. Configure environment variables in `.env`:

   ```env
   DATABASE_URL=postgres://testuser:pass@localhost:5432/testdb
   JWT_SECRET_KEY=your_super_secret_key
   SERVICE_PORT=8080
   ```

3. Prepare SQLx offline mode with validation:

   ```bash
   cargo sqlx prepare --check
   ```

4. Run the server locally:

   ```bash
   cargo run
   ```

---

## 🚀 Usage

### Authenticate and Call Protected API

1. Login to obtain a JWT token:

   ```bash
   curl -X POST http://localhost:8080/auth/login \
     -H "Content-Type: application/json" \
     -d '{"client_id":"apitest01","client_secret":"test_password"}'
   ```

2. Use the returned `token` to access protected endpoints:

   ```bash
   curl http://localhost:8080/user -H "Authorization: Bearer $token"
   ```

### API Documentation

Open [http://localhost:8080/docs](http://localhost:8080/docs) in your browser for Swagger UI.

- Authenticate via `/auth/login` (POST) with JSON payload:

  ```json
  {
    "client_id": "apitest01",
    "client_secret": "test_password"
  }
  ```

- Copy the returned JWT token.
- Click the 🔒 Authorize button in Swagger UI and paste the token to authorize requests.

---

## 🧠 Domain-Driven Design & Architecture

### Domain Models

- Plain Rust structs and enums represent domain entities.
- Business logic resides within domain services (`domain/service.rs`).

### Use Case Isolation & Dependency Inversion

- Domain service traits define business contracts.
- Concrete implementations live in `services.rs`, constructed via factory methods.
- `bootstrap.rs` wires services and builds `AppState` for dependency injection.

### Interface Layer (Axum)

- Route handlers accept DTOs, invoke domain logic, and return serialized responses.
- Each domain owns its own `routes.rs` and `handlers.rs`.
- Supports asynchronous multipart file uploads with validation.
- Secure file serving validates user permissions and sanitizes file paths.

### DTOs & Validation

- Request and response DTOs reside in each domain's `dto.rs`.
- Explicit mapping between DTOs and domain models.
- Uses `serde` and optionally the [`validator`](https://docs.rs/validator) crate for input validation.

---

## 🧱 Database Schema

See the `db-seed/` directory for table definitions and sample data.  
The database structure is illustrated in the Entity Relationship Diagram:

![ER Diagram](./ERD.png)

---

## 📚 API Documentation

- Swagger UI is available at `/docs` (powered by Utoipa).
- DTOs and endpoints are annotated for OpenAPI specification.

---

## 📦 API Response Format

All endpoints return a consistent JSON envelope:

```json
{
  "status": 200,
  "message": "success",
  "data": { ... }
}
```

Implemented as:

- `ApiResponse<T>` – generic response wrapper
- `RestApiResponse<T>` – wrapper implementing Axum's `IntoResponse` trait

See definitions in `common/dto.rs`.

---

## 🧪 Testing

- Unit tests cover domain logic and core components.
- Integration tests exercise HTTP endpoints and database interactions.
- Use `#[tokio::test]` and `tower::ServiceExt` for HTTP simulation.
- Test assets and helpers are located in the `tests/` directory.

---

## 🚨 Error Handling

- Centralized `AppError` enum implements `IntoResponse`.
- Errors map to appropriate HTTP status codes with JSON structure, e.g.:

```json
{
  "status": 400,
  "message": "Invalid request data",
  "data": null
}
```

---

## 🧪 Environment Configuration

Configure via `.env` at the project root.  
Set database URL, JWT secret, service port, and asset settings.

Example `.env`:

```env
DATABASE_URL=postgres://testuser:pass@localhost:5432/testdb
JWT_SECRET_KEY=your_super_secret_key
SERVICE_PORT=8080
```

---

## 📡 OpenTelemetry (Tracing & Metrics)

This project supports distributed tracing, logging, and metrics via OpenTelemetry.

### Setup Jaeger Collector:

```bash
docker run --rm --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 5778:5778 \
  -p 9411:9411 \
  jaegertracing/jaeger:2.6.0
```

Access the Jaeger UI at [http://localhost:16686](http://localhost:16686).

### Enable OpenTelemetry Feature:

- Run with OpenTelemetry:

  ```bash
  cargo run --features opentelemetry
  ```

- Build with OpenTelemetry:

  ```bash
  cargo build --features opentelemetry
  ```

For details, see the [Jaeger Getting Started guide](https://www.jaegertracing.io/docs/2.6/getting-started/).

---

## 🚧 Roadmap

- gRPC support for machine-to-machine APIs
- Role-Based Access Control (RBAC)
- Expanded documentation including schema and infrastructure insights

---

## 🤝 Contributing

Contributions are welcome! Feel free to open issues, suggest improvements, or submit pull requests.  
See the roadmap above for ideas or propose your own. Let's build something great together 🚀

---

## 📄 License & Resources

- MIT License. See [LICENSE](./LICENSE) for details.

### Useful Links

- [Axum](https://docs.rs/axum)
- [SQLx](https://docs.rs/sqlx)
- [Utoipa (OpenAPI)](https://docs.rs/utoipa)
- [Tokio](https://tokio.rs/)
- [Validator (crate)](https://docs.rs/validator)

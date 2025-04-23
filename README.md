# Rust Axum Clean Demo

Rust Axum Clean Demo – Basic API Template

This document outlines a Rust API Server Sample Demo using Axum and SQLx. It integrates Clean Architecture principles, domain-driven design, repository patterns, Swagger documentation, and robust testing.

## ✨ Features

- ✅ Clean Architecture with layered domain separation
- ✅ Axum-based HTTP server with modular routing
- ✅ SQLx integration with compile-time query checking
- ✅ JWT-based authentication and protected routes
- ✅ File upload and secure asset serving
- ✅ Swagger UI docs via utoipa

## 🛠 Getting Started

### Prerequisites

- Rust (latest stable)
- MySQL or MariaDB
- Docker and Docker Compose (optional, for containerized setup)

#### Using Docker Compose

You can use Docker Compose to build and run the application and its dependencies:

```bash
docker-compose up --build
```

To stop the services and remove containers, networks, and images:

```bash
docker-compose down --rmi all
```

#### Not Using Docker Compose

1. **Create database tables**  
   Navigate to the `db-seed` directory and execute the SQL scripts in order:

   ```bash
   cd db-seed
   mysql -u <user> -p <database> < 01-tables.sql
   mysql -u <user> -p <database> < 02-seed.sql
   ```

2. **Configure environment**

   Update `.env` with:

   ```env
   DATABASE_URL=mysql://user:password@localhost/clean_axum_demo
   JWT_SECRET_KEY=your_super_secret_key
   SERVICE_PORT=8080
   ```

   Ensure `.env.test` has test-specific values (e.g., test DB).

3. **Prepare SQLx (Offline Compilation)**  
   Generate offline metadata:

   ```bash
   cargo sqlx prepare
   ```

4. **Running the Application Locally**

   ```bash
   cargo run
   ```

#### Usage

1. Example Login & Protected‑API Usage:

   - Send a login request:

     ```bash
     curl -X POST http://localhost:8080/auth/login \
       -H "Content-Type: application/json" \
       -d '{"client_id":"apitest01","client_secret":"test_password"}'
     ```

   - Copy the `token` value from the JSON response.
   - Call a protected endpoint:

     ```bash
     curl http://localhost:8080/user \
       -H "Authorization: Bearer <token>"
     ```

2. View the API documentation:
   Open your browser and go to [http://localhost:8080/docs](http://localhost:8080/docs).
3. Access protected endpoints:

   - Authenticate by sending a `POST` request to `/auth/login` (e.g., via Swagger UI or curl).

     ```json
     {
       "client_id": "apitest01",
       "client_secret": "test_password"
     }
     ```

     - Copy the returned JWT token.

   - In Swagger UI, click the 🔒 Authorize button and paste `<jwt>` to authorize requests.

### 📦 Project Structure: Layered + Modular

Each domain module (e.g., `auth`, `device`, `file`, `user`) follows a consistent structure:

- `dto.rs`: Defines the DTO (Data Transfer Object) layer for the domain.
- `handlers.rs`: Defines the HTTP handler layer for the domain.
- `services.rs`: Contains domain service logic and use cases.
- `model.rs`: Defines the domain model and business logic layer.
- `queries.rs`: Defines raw SQLx query implementations.
- `repository.rs`: Implements the repository pattern for database operations.
- `routes.rs`: Defines HTTP route configuration for the domain.

Organize your project by **domain-first modularity**. Each domain encapsulates its own types, services, database logic, and routing. This ensures high cohesion, better testability, and easier maintenance.

Recommended structure:

```plain
├── assets                          # Static assets used by the application
│   ├── private                     # Private user files (e.g., uploads)
│   │   └── profile_picture
│   │       ├── cat.png
│   │       ├── cat(1).png
│   │       ├── images.jpeg
│   │       └── mario_PNG52.png
│   └── public                      # Publicly accessible static files
│       └── images.jpeg
├── Cargo.lock
├── Cargo.toml
├── db-seed
│   ├── 01-tables.sql
│   └── 02-seed.sql
├── LICENSE
├── README.md
├── .env                 # Environment variables for development/runtime configuration
├── .env.test            # Environment variables used specifically during test execution
├── src
│   ├── app.rs               # Axum router setup & middleware configuration
│   ├── auth
│   │   ├── dto.rs
│   │   ├── handlers.rs
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   ├── queries.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── services.rs
│   ├── common
│   │   ├── app_state.rs         # Global application state container shared across routes
│   │   ├── config.rs            # Environment and runtime configuration loading
│   │   ├── dto.rs               # Shared API response wrapper DTOs (e.g., ApiResponse<T>)
│   │   ├── error.rs             # Centralized error definitions and handling via AppError
│   │   ├── hash_util.rs         # Utility for password hashing and verification (e.g., bcrypt)
│   │   ├── jwt.rs               # JWT creation, decoding, and validation logic
│   │   ├── mod.rs
│   │   └── ts_format.rs         # Custom timestamp formatting for JSON serialization
│   ├── device
│   │   ├── dto.rs
│   │   ├── handlers.rs
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   ├── queries.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── services.rs
│   ├── file
│   │   ├── dto.rs
│   │   ├── handlers.rs
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   ├── queries.rs
│   │   ├── repository.rs
│   │   ├── routes.rs
│   │   └── services.rs
│   ├── lib.rs
│   ├── main.rs
│   └── user
│       ├── dto.rs
│       ├── handlers.rs
│       ├── mod.rs
│       ├── model.rs
│       ├── queries.rs
│       ├── repository.rs
│       ├── routes.rs
│       └── services.rs
└── tests                        # Integration tests using real endpoints and data
    ├── asset                    # Static files used in test scenarios (e.g., file uploads)
    │   ├── cat.png
    │   └── mario_PNG52.png
    ├── test_auth_routes.rs
    ├── test_device_routes.rs
    ├── test_helpers.rs
    └── test_user_routes.rs
```

---

### 📦 API Response Format

All API endpoints return a consistent JSON envelope, defined using the `ApiResponse<T>` and `RestApiResponse<T>` types:

```json
{
  "status": 200,
  "message": "success",
  "data": {
    ... // actual payload here
  }
}
```

#### `ApiResponse<T>`

A generic structure representing success or error states for API calls:

```rust
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}
```

- `status`: HTTP status code (e.g. 200, 404)
- `message`: Human-readable description
- `data`: Optional actual response payload

#### `RestApiResponse<T>`

A convenience wrapper for returning `ApiResponse<T>` directly from handlers:

```rust
pub struct RestApiResponse<T>(pub ApiResponse<T>);
```

This type implements `IntoResponse` for Axum, allowing clean usage:

```rust
Ok(RestApiResponse::success(payload))
Ok(RestApiResponse::failure(404, "Item not found"))
```

Use `ApiResponse<T>` for Swagger documentation (`utoipa`), and return `RestApiResponse<T>` from handler functions.

---

### 🧪 Environment Configuration

Test configuration is managed via a `.env` file at the root. This includes:

- `DATABASE_URL`: Connection string for the MariaDB test database.
- `SERVICE_PORT`, `SERVICE_HOST`: Host and port used for the Axum server.
- `JWT_SECRET_KEY`: Secret key used to sign and verify JWT tokens.
- `ASSETS_*`: Configuration for asset storage, URL mapping, and allowed extensions.
- `ASSET_MAX_SIZE`: Upload size limit (in bytes) for files.
- `ASSET_ALLOWED_EXTENSIONS`: Pipe-separated list of allowed file extensions for uploads.

Example:

```shell
DATABASE_URL=mysql://user:pass@localhost/test_db
```

---

### 🧠 Domain-Driven Design

- Domain models are plain Rust types (structs/enums).
- Business logic is encapsulated in domain services or model methods.
- Free of frameworks or database concerns.

---

### 🔄 Use Case Isolation & Dependency Inversion

- Each operation is defined in a service or use case.
- Use traits for repositories, injected into handlers via `Arc<T>`.
- Infrastructure implements these traits, enabling mocking and isolation.

---

### 🔌 Infrastructure Layer (SQLx + MariaDB)

- Database access via `sqlx` with strongly typed queries.
- UUIDs stored as `CHAR(36)`, fixed length for performance.
- Database queries are implemented under each domain's module `repository.rs`, `queries.rs` (e.g., `user/repository.rs`, `user/queries.rs`).

---

### 🧭 When to Use `QueryBuilder` vs. Static Queries

Use `QueryBuilder` when:

- You need to dynamically build SQL (e.g., optional filters, partial updates)
- You're performing batch inserts or updates (e.g., `INSERT ... ON DUPLICATE KEY UPDATE`)
- You want flexibility at runtime without hardcoding all query clauses

Avoid `QueryBuilder` and use `sqlx::query!` or `query!` macro when:

- Your queries are static and known at compile time
- You want compile-time checking for SQL correctness and type safety
- You value performance and simpler syntax over runtime flexibility

---

### 🧱 Sample Database Tables

Example schemas to support the domain modules:

#### `users` Table

| Column      | Type         | Description                  |
| ----------- | ------------ | ---------------------------- |
| id          | CHAR(36)     | Primary key (auto-generated) |
| username    | VARCHAR(64)  | Unique username              |
| email       | VARCHAR(128) | User's email address         |
| created_by  | CHAR(36)     | ID of the creator            |
| created_at  | TIMESTAMP    | Account creation time        |
| modified_by | CHAR(36)     | ID of the last modifier      |
| modified_at | TIMESTAMP    | Last modification time       |

#### `user_auth` Table

| Column        | Type         | Description                             |
| ------------- | ------------ | --------------------------------------- |
| user_id       | CHAR(36)     | Primary key, references `users(id)`     |
| password_hash | VARCHAR(255) | Hashed password (e.g., bcrypt)          |
| created_at    | TIMESTAMP    | Timestamp when credentials were created |
| modified_at   | TIMESTAMP    | Timestamp when credentials were updated |

#### `devices` Table

| Column        | Type         | Description                           |
| ------------- | ------------ | ------------------------------------- |
| id            | CHAR(36)     | Primary key (auto-generated)          |
| user_id       | CHAR(36)     | Foreign key to `users(id)`            |
| name          | VARCHAR(128) | Device name                           |
| status        | VARCHAR(32)  | Current status (e.g., active)         |
| device_os     | VARCHAR(16)  | Operating system (`Android` or `iOS`) |
| registered_at | TIMESTAMP    | When the device was registered        |
| created_by    | CHAR(36)     | ID of the creator                     |
| created_at    | TIMESTAMP    | Account creation time                 |
| modified_by   | CHAR(36)     | ID of the last modifier               |
| modified_at   | TIMESTAMP    | Last modification time                |

##### `devices.status` Possible Values

| Value            | Description                                 |
| ---------------- | ------------------------------------------- |
| `active`         | Device is operational and in use            |
| `inactive`       | Device is offline or unused                 |
| `pending`        | Device is awaiting approval or registration |
| `blocked`        | Device is blacklisted or suspended          |
| `decommissioned` | Device has been retired or archived         |

#### `uploaded_files` Table

| Column             | Type         | Description                                          |
| ------------------ | ------------ | ---------------------------------------------------- |
| id                 | CHAR(36)     | Primary key, auto-generated UUID                     |
| user_id            | CHAR(36)     | Foreign key referencing `users(id)`                  |
| file_name          | VARCHAR(128) | Stored/generated file name on your storage system    |
| origin_file_name   | VARCHAR(128) | Original file name from user upload                  |
| file_relative_path | VARCHAR(256) | Relative file path on the storage system             |
| file_url           | VARCHAR(256) | URL to access the file                               |
| content_type       | VARCHAR(64)  | MIME type of the file                                |
| file_size          | INT UNSIGNED | File size in bytes                                   |
| file_type          | VARCHAR(16)  | Type of file (e.g., image, video)                    |
| created_by         | CHAR(36)     | ID of the creator                                    |
| created_at         | TIMESTAMP    | Timestamp of creation (default is current timestamp) |
| modified_by        | CHAR(36)     | ID of the last modifier                              |
| modified_at        | TIMESTAMP    | Timestamp of last update (auto-updated)              |

---

### 🌐 Interface Layer (Axum)

- Thin route handlers that:
  - Accept DTOs
  - Call domain/application services
  - Return serialized response DTOs
- Each domain contains its own module with `routes.rs` and `handlers.rs` files.
- **Multipart File Upload:** Some endpoints, such as `create_user`, accept file uploads via the `Multipart` extractor. This enables asynchronous processing of each form part—whether it's file data or other form fields—ensuring efficient streaming and robust validation. Each file should be verified for content type, sanitized to prevent directory traversal or injection attacks, and stored securely. This approach not only enhances flexibility in handling user input but also bolsters the system’s security posture.
- **Protected File Serving:** Implement endpoints like `serve_protected_file` to securely serve static files or resources. This handler should verify user permissions through tokens, session validations, or appropriate authentication headers, ensuring that only authenticated users can access the protected files. Additionally, it should enforce file path sanitization to prevent directory traversal attacks and may include caching strategies for performance optimization.

---

### 🧾 DTOs & Mapping

- Request and response models live in each domain's `dto.rs` module (e.g., `user/dto.rs`).
- Explicit conversion between domain models and DTOs.
- Validation (e.g., enums, formats) handled at DTO level or via `serde`.
- For enhanced input validation, consider utilizing the [`validator`](https://docs.rs/validator) crate. This crate leverages the `#[derive(Validate)]` procedural macro to annotate DTO fields with constraints (e.g., ensuring strings are not empty, emails are valid, etc.). For example:

  ```rust
  use validator::Validate;

  #[derive(Debug, Validate)]
  pub struct CreateUserDto {
      #[validate(length(min = 1, message = "Username is required"))]
      pub username: String,

      #[validate(email(message = "Invalid email format"))]
      pub email: String,
  }
  ```

After deserialization, call `.validate()` on the DTO instance to enforce these rules and handle any errors gracefully.

---

### 📚 API Documentation with utoipa

- DTOs annotated with `#[derive(ToSchema)]`
- Endpoints documented via `#[openapi(...)]`
- Define `#[derive(OpenApi)]` in each domain or aggregate routes in a central documentation module.
- `utoipa-swagger-ui` serves Swagger docs at `/docs`

---

### ✅ Testing Strategy

- **Unit tests**: Verify domain logic.
- **Integration tests**: Use Axum and a test DB.
- `#[tokio::test]` + `tower::ServiceExt` for realistic HTTP simulation.

---

### 🚨 Centralized Error Handling

- Define a top-level `AppError` enum implementing `std::error::Error` and `IntoResponse` for Axum.
- Categorize errors (e.g., `DatabaseError`, `ValidationError`, `NotFound`).
- Use `thiserror` for ergonomic declarations and matching.
- Map these errors to appropriate HTTP status codes using `impl IntoResponse`.
- Ensure consistent JSON error structure across all endpoints for better DX and frontend compatibility.

---

## 🚧 Roadmap

Here’s a high-level roadmap for evolving the architecture and infrastructure of this project:

### 1. 🛠️ Hexagonal Architecture (Ports & Adapters)

**Goal:** Decouple infrastructure concerns from core domain logic for greater flexibility and testability.

**Steps:**

- Create separate crates for:
  - `domain`: core business rules and types
  - `infra`: SQLx, JWT, File I/O, etc.
  - `app`: service orchestration (use cases)
  - `web`: HTTP adapter (Axum)
- Define all repository/service traits (ports) in the `domain` layer.
- Implement adapters for HTTP, database, and storage that live in `infra`.
- Enable future support for gRPC, CLI, or background workers.

---

### 2. 📈 OpenTelemetry for Tracing & Metrics

**Goal:** Add observability with distributed tracing, logs, and metrics.

**Steps:**

- Integrate `tracing`, `tracing_subscriber`, and `opentelemetry` crates.
- Export traces to tools like Jaeger or Zipkin.
- Add span instrumentation to Axum routes, DB queries, and service functions.
- Enable structured logs for better debugging.

---

### 3. 🔄 Migrate from MySQL to PostgreSQL

**Goal:** Adopt a more feature-rich and scalable relational database.

**Steps:**

- Update `DATABASE_URL` to use `postgres://`
- Switch to native `UUID` columns instead of `CHAR(36)`
- Use PostgreSQL-specific features like `jsonb`, `ON CONFLICT DO UPDATE`, etc.
- Update `.sql` seed and table definitions accordingly

---

### ✅ Optional Enhancements

| Feature                     | Benefit                                            |
| --------------------------- | -------------------------------------------------- |
| ✨ gRPC support via `tonic` | Enable machine-to-machine API communication        |
| 📦 Workspace structure      | Modularize code into `core`, `web`, `infra` crates |
| 🔐 Role-based access (RBAC) | Control access to endpoints based on user roles    |

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

## 🔗 Useful Links

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Utoipa (OpenAPI)](https://docs.rs/utoipa)
- [Tokio](https://tokio.rs/)

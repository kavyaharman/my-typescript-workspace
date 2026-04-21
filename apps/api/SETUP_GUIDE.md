# Rust Hello World API - Complete Setup Guide

## 📁 Project Structure
```
rust-hello-api/
├── Cargo.toml          # Project metadata & dependencies
├── src/
│   └── main.rs         # Main application code
└── .gitignore          # Files to ignore in git
```

---

## 🔧 Understanding the Setup

### **1. Cargo.toml (Project Configuration)**
```toml
[package]
name = "rust-hello-api"
version = "0.1.0"
edition = "2021"              # Rust version (2021 is latest stable)

[dependencies]
actix-web = "4"               # Web framework for HTTP
tokio = { version = "1", features = ["full"] }  # Async runtime
serde = { version = "1.0", features = ["derive"] }  # JSON serialization
serde_json = "1.0"            # JSON support
```

**What each dependency does:**
- **actix-web**: Fast, production-ready web framework (like Express.js)
- **tokio**: Async runtime (allows non-blocking I/O, like Node.js)
- **serde**: Serializes/deserializes Rust structs to JSON
- **serde_json**: JSON utilities

### **2. src/main.rs (Application Code)**

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Define a JSON response struct
#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
    status: String,
}

// Simple hello world endpoint
async fn hello() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Hello with name parameter
async fn hello_name(name: web::Path<String>) -> impl Responder {
    let response = HelloResponse {
        message: format!("Hello, {}!", name.into_inner()),
        status: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Health check endpoint
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/hello/{name}", web::get().to(hello_name))
            .route("/health", web::get().to(health))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**Code Breakdown:**

```rust
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
```
Imports the web framework components.

```rust
#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
    status: String,
}
```
**Struct**: Defines JSON response format. The `#[derive(...)]` automatically generates serialization code.

```rust
async fn hello() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}
```
**Handler function**: 
- `async` = non-blocking (handles multiple requests)
- `impl Responder` = returns something that can be converted to HTTP response
- Returns JSON with 200 OK status

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
```
**Main entry point**:
- `#[actix_web::main]` macro = sets up the async runtime
- `async fn` = allows `await` for async operations

```rust
HttpServer::new(|| {
    App::new()
        .route("/", web::get().to(hello))
        .route("/hello/{name}", web::get().to(hello_name))
        .route("/health", web::get().to(health))
})
.bind("127.0.0.1:8080")?
.run()
.await
```
**Server setup**:
1. Creates a new HTTP server
2. Defines 3 routes:
   - `GET /` → `hello` function
   - `GET /hello/{name}` → `hello_name` function with URL parameter
   - `GET /health` → `health` function
3. Binds to localhost:8080
4. Runs indefinitely

---

## 🚀 Running the Project

### **Prerequisites**
- Install Rust: https://rustup.rs/

### **Commands**

1. **Navigate to project:**
   ```powershell
   cd c:\Users\kavyn\Desktop\rust-hello-api
   ```

2. **Build & run:**
   ```powershell
   cargo run
   ```
   (First run takes ~30 seconds as it downloads & compiles dependencies)

3. **Access the API:**
   - http://localhost:8080/ → `{"message": "Hello, World!", "status": "success"}`
   - http://localhost:8080/hello/Alice → `{"message": "Hello, Alice!", "status": "success"}`
   - http://localhost:8080/health → `{"status": "healthy"}`

### **Development Commands**
```powershell
cargo build              # Compile only
cargo run --release     # Optimized build (faster runtime)
cargo test              # Run tests
cargo fmt               # Format code
cargo clippy            # Linting suggestions
```

---

## 🔑 Key Rust Concepts in This Project

| Concept | Explanation |
|---------|-------------|
| `async/await` | Non-blocking code execution |
| `let binding` | Declare immutable variables |
| `to_string()` | Convert to String type |
| `impl Responder` | Generic return type trait |
| `#[derive(...)]` | Auto-generate code (like decorators) |
| `?` operator | Error handling shorthand |

---

## 📊 Adding More Features

### **Add a POST endpoint that accepts JSON:**
```rust
#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

async fn create_item(item: web::Json<CreateItem>) -> impl Responder {
    HttpResponse::Created().json(serde_json::json!({"id": 1, "name": item.name}))
}

// In main():
.route("/items", web::post().to(create_item))
```

### **Add Path Parameters:**
```rust
#[derive(web::Path)]
struct ItemPath {
    id: u32,
}

async fn get_item(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({"id": id}))
}

// In main():
.route("/items/{id}", web::get().to(get_item))
```

### **Add Query Parameters:**
```rust
#[derive(Deserialize)]
struct QueryParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

async fn list_items(query: web::Query<QueryParams>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"limit": query.limit, "offset": query.offset}))
}

// In main():
.route("/items", web::get().to(list_items))
// Usage: /items?limit=10&offset=0
```

---

## 🐛 Common Errors & Fixes

| Error | Fix |
|-------|-----|
| `error: failed to resolve: use of undeclared crate or module` | Run `cargo build` to fetch dependencies |
| `Address already in use` | Change port in `.bind("127.0.0.1:8080")` |
| `cannot find function/struct` | Check imports with `use` statements |
| `expected struct, found enum` | Check type mismatches in responses |

---

## 📚 Useful Resources
- Actix-web Docs: https://actix.rs/
- Rust Book: https://doc.rust-lang.org/book/
- Cargo Manual: https://doc.rust-lang.org/cargo/
- Serde Docs: https://serde.rs/

---

## Quick Reference: HTTP Methods

```rust
// GET - Retrieve data
.route("/users", web::get().to(get_users))

// POST - Create data
.route("/users", web::post().to(create_user))

// PUT - Update entire resource
.route("/users/{id}", web::put().to(update_user))

// DELETE - Remove data
.route("/users/{id}", web::delete().to(delete_user))

// PATCH - Partial update
.route("/users/{id}", web::patch().to(patch_user))
```

---

**Last Updated:** April 17, 2026

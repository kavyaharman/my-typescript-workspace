use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
struct User{
    name: String,
    age: i32,
    admin: bool,
    email: String,
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn hello() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn hello_name(name: web::Path<String>) -> impl Responder {
    let response = HelloResponse {
        message: format!("Hello, {}!", name.into_inner()),
        status: "success".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn users() -> impl Responder {
    let users = vec![
        serde_json::json!({"id": 1, "name": "Alice", "email": "alice@example.com"}),
        serde_json::json!({"id": 2, "name": "Bob", "email": "bob@example.com"}),
        serde_json::json!({"id": 3, "name": "Charlie", "email": "charlie@example.com"}),
    ];
    HttpResponse::Ok().json(users)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/health", web::get().to(health))
            .route("/users",web::get().to(users))
            .route("/users", web::post().to(create_user))
            .route("/hello/{name}", web::get().to(hello_name))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

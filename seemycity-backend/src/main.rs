use actix_web::{web, App, HttpServer};

// Declare modules corresponding to our file structure
pub mod api;
pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;

// Import the specific handler function we need
use crate::handlers::hello;

// The main function is the entry point of the application.
// The #[actix_web::main] macro sets up the Tokio async runtime needed by Actix.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting on http://127.0.0.1:4000");

    // HttpServer::new creates a new application instance.
    // The closure passed to `new` configures the application, defining routes and middleware.
    HttpServer::new(|| {
        App::new()
            // .service now uses the imported handlers::hello
            .service(hello)
    })
    // .bind specifies the address and port to listen on.
    .bind(("127.0.0.1", 4000))?
    // .run starts the server and waits for incoming connections.
    .run()
    .await
}

use actix_web::{get, Responder, HttpResponse};

// Handler for the root path ("/").
// Mark the function as `pub` so it can be imported and used in `main.rs`.
#[get("/")]
pub async fn hello() -> impl Responder {
    // Use HttpResponse for a more explicit response
    HttpResponse::Ok().body("Hello from SeeMyCity Backend! (via handlers module)")
}

// We can add other handlers here later, e.g., for /api/municipalities
// pub mod municipalities; // If we create handlers/municipalities.rs
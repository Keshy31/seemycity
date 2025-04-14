use actix_web::{get, web, HttpResponse, Responder};

pub mod municipalities;

// Handler for the root path
#[get("/")]
pub async fn root_handler() -> impl Responder {
    HttpResponse::Ok().body("Hello from SeeMyCity Backend! (via handlers module)")
}

// Function to configure routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(municipalities::get_municipality_detail_handler)
            // Add other handlers here
    );
}
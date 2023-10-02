use actix_web::{rt::System, web, App, HttpResponse, HttpServer};
use crate::db::db;

pub async fn start_api() {
    let address = "127.0.0.1";
    let port = 8000;
    let api = HttpServer::new(|| App::new().configure(configure_routes)).bind(format!("{}:{}", address, port))
        .expect("Failed to bind to address");
    println!("Server started at http://{}:{}", address, port);
    api.run().await.expect("Failed to start server");
}

pub fn run() {
    System::new().block_on(start_api());
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .route("/ping", web::get().to(ping))
        .route("/books", web::get().to(get_books))
    );
}

async fn ping() -> HttpResponse {
    println!("Books: {:?}", db::read_books().expect("Failed to read books"));
    HttpResponse::Ok().body("Connected!")
}

async fn get_books() -> HttpResponse {
    HttpResponse::Ok().json(db::read_books().expect("Failed to read books"))
}

use crate::db::db;
use crate::model::book::Book;
use actix_web::{rt::System, web, App, HttpResponse, HttpServer};

use super::protocols::BookRequest;

pub async fn start_api() {
    let address = "127.0.0.1";
    let port = 8088;
    let api = HttpServer::new(|| App::new().configure(configure_routes))
        .bind(format!("{}:{}", address, port))
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
            .route("/books", web::post().to(add_book)),
    );
}

async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Connected!")
}

async fn get_books() -> HttpResponse {
    HttpResponse::Ok().json(db::read_books().expect("Failed to read books"))
}

async fn add_book(request: web::Json<BookRequest>) -> HttpResponse {
    let request = request.into_inner();
    let book = Book::new(
        request.title,
        request.author,
        request.isbn,
        request.year,
        request.pages,
        request.reading_status,
        request.start_date,
        request.end_date,
    );
    HttpResponse::Created().json(db::add_book(book).expect("Failed to add book"))
}

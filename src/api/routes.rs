use crate::db::db;
use crate::model::book::Book;
use actix_web::{rt::System, web, App, HttpResponse, HttpServer};

use super::protocols::BookRequest;

const DB_FILENAME: &str = ".books.json";

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
            .route("/books", web::post().to(add_book))
            .route("/books/{id}", web::get().to(get_book))
            .route("/books/{id}", web::delete().to(delete_book))
            .route("/books/{id}", web::put().to(update_book))
            .route("/books/{id}/completed", web::patch().to(complete_book))
            .route("/books/{id}/reading", web::patch().to(reading_book)),
    );
}

/// Ping endpoint for health check
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Connected!")
}

async fn get_books() -> HttpResponse {
    HttpResponse::Ok().json(db::read_books(DB_FILENAME).expect("Failed to read books"))
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
    HttpResponse::Created().json(db::add_book(book, DB_FILENAME).expect("Failed to add book"))
}

async fn get_book(id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    HttpResponse::Ok().json(db::get_book(id, DB_FILENAME).expect("Failed to get book"))
}

async fn delete_book(id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    HttpResponse::Ok().json(db::delete_book(id, DB_FILENAME).expect("Failed to delete book"))
}

async fn update_book(request: web::Json<BookRequest>, id: web::Path<String>) -> HttpResponse {
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
    let id = id.into_inner();
    let book = Book::new_with_id(id, book);
    HttpResponse::Ok().json(db::update_book(book, DB_FILENAME).expect("Failed to update book"))
}

async fn complete_book(id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    HttpResponse::Ok().json(db::complete_book(id, DB_FILENAME).expect("Failed to complete book"))
}

async fn reading_book(id: web::Path<String>) -> HttpResponse {
    let id = id.into_inner();
    HttpResponse::Ok().json(db::reading_book(id, DB_FILENAME).expect("Failed to reading book"))
}

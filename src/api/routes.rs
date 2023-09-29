use actix_web::{rt::System, web, App, HttpResponse, HttpServer};

pub async fn start_api() {
    let address = "127.0.0.1";
    let port = 8000;
    let api = HttpServer::new(|| App::new().route("/api/hello", web::get().to(hello)))
        .bind(format!("{}:{}", address, port))
        .expect("Failed to bind to address");
    println!("Server started at http://{}:{}", address, port);
    api.run().await.expect("Failed to start server");
}

pub fn run() {
    System::new().block_on(start_api());
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

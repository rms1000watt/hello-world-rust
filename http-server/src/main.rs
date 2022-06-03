use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {name}!")
}

#[get("/hi/{name}")]
async fn greet2(name: web::Path<String>) -> impl Responder {
    format!("hi {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // TODO: undo ? with explicit match
    // TODO: cli tool for passing in args
    HttpServer::new(|| App::new().service(greet).service(greet2))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

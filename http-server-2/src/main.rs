use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {name}!")
}

#[get("/hi/{name}")]
async fn greet2(name: web::Path<String>) -> impl Responder {
    format!("hi {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: cli tool for passing in args

    let app = || App::new().service(greet).service(greet2);
    let bind = match HttpServer::new(app).bind(("127.0.0.1", 8080)) {
        Ok(bind) => bind,
        Err(e) => {
            println!("failed bind {}", e);
            return Err(e);
        }
    };

    return bind.workers(2).run().await;
}

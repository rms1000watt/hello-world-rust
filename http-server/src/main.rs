use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;

#[derive(Parser)]
struct CLI {
    #[clap(short, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, parse(try_from_str), default_value = "8080")]
    port: u16,

    #[clap(short, parse(try_from_str), default_value = "2")]
    workers: usize,
}

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
    let cli = CLI::parse();
    println!("Listening on: host={} port={}", cli.host, cli.port);

    let app = || App::new().service(greet).service(greet2);
    let bind = match HttpServer::new(app).bind((cli.host, cli.port)) {
        Ok(bind) => bind,
        Err(e) => {
            eprintln!("Failed binding to socket: {}", e);
            return Err(e);
        }
    };

    return bind.workers(cli.workers).run().await;
}

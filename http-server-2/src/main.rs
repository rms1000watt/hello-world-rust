use actix_web::{get, web, App, HttpServer, Responder};
use clap::Parser;
use std::error;

#[derive(Parser)]
struct CLI {
    #[clap(short, default_value = "127.0.0.1")]
    host: String,

    // TODO: how to actually handle ints
    #[clap(short, default_value = "8080")]
    port: String,

    // TODO: how to actually handle ints
    #[clap(short, default_value = "2")]
    workers: String,
}

struct Config {
    host: String,
    port: u16,
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
    let config = match get_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed getting config: {}", e);
            // TODO: Don't panic, handle the error gracefully... not sure how tho
            panic!();
        }
    };

    println!("Listening on: host={} port={}", config.host, config.port);

    let app = || App::new().service(greet).service(greet2);
    let bind = match HttpServer::new(app).bind((config.host, config.port)) {
        Ok(bind) => bind,
        Err(e) => {
            eprintln!("Failed binding to socket: {}", e);
            return Err(e);
        }
    };

    return bind.workers(config.workers).run().await;
}

fn get_config() -> Result<Config, Box<dyn error::Error>> {
    let cli = CLI::parse();

    let port: u16 = match cli.port.parse() {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Error: non-numeric port provided as option: {}", e);
            return Err(Box::new(e));
        }
    };

    let workers: usize = match cli.workers.parse() {
        Ok(workers) => workers,
        Err(e) => {
            eprintln!("Error: non-numeric workers provided as option: {}", e);
            return Err(Box::new(e));
        }
    };

    let config = Config {
        host: cli.host,
        port: port,
        workers: workers,
    };

    return Ok(config);
}

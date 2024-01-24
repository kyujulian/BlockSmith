use actix_cors::Cors;
use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use chain::{self, chain::Blockchain, wallet}; // bad naming
use std::net::TcpListener;

use tracing::{self, debug, info};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/mine")]
async fn mine() -> impl Responder {
    HttpResponse::Ok().body("Mining a new block")
}
#[tracing::instrument]
#[get("/wallet/new")]
async fn new_wallet() -> impl Responder {
    let wallet = wallet::Wallet::generate_new();
    let res_body = serde_json::to_string(&wallet);
    match res_body {
        Ok(body) => {
            info!("Wallet created");
            debug!("{}", format!("New wallet: {}", body));
            HttpResponse::Ok().body(body)
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize wallet"),
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let root_wallet = wallet::Wallet::generate_new();
    let address = root_wallet.address();

    let blockchain = Blockchain::new(address);

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors).service(hello).service(new_wallet)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
#[cfg(test)]
mod tests {}

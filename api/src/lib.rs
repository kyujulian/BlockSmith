use actix_cors::Cors;
use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use chain::{self, chain::Blockchain, wallet}; // bad naming
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tracing::{self, debug, info};

#[derive(Deserialize, Serialize)]
struct TransactionRequest {
    sender_address: String,
    recipient_address: String,
    value: f32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/mine")]
async fn mine(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    match data.lock() {
        Ok(mut chain) => {
            chain.mine();

            HttpResponse::Ok().body("Mining a new block")
        }
        Err(_) => HttpResponse::ExpectationFailed().body("Failed to lock data"),
    }
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

#[tracing::instrument]
#[post("/transaction/new")]
async fn new_transaction(
    req_body: String,
    data: web::Data<Arc<Mutex<Blockchain>>>,
) -> impl Responder {
    info!("New transaction");

    let transaction: Result<TransactionRequest, _> = serde_json::from_str(&req_body);
    match transaction {
        Ok(tx) => {
            let mut chain = data.lock().expect("Failed to mutex lock chain");

            chain.add_transaction(&tx.sender_address, &tx.recipient_address, tx.value);
            HttpResponse::Ok().body("Transaction added successfully")
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to serialize transaction")
        }
    }
}

#[tracing::instrument]
#[get("/chain")]
async fn get_chain(data: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let chain = data.lock().unwrap().chain();

    let res_body = serde_json::to_string(&chain);
    match res_body {
        Ok(body) => {
            info!("Query for chain succeeded");
            debug!("{}", format!("Chain: {}", body));
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

    let shared_blockchain = Arc::new(Mutex::new(Blockchain::new(address)));

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(shared_blockchain.clone()))
            .service(hello)
            .service(new_wallet)
            .service(get_chain)
            .service(mine)
            .service(new_transaction)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
#[cfg(test)]
mod tests {}

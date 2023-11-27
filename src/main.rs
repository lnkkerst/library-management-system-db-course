mod controllers;
#[allow(warnings)]
mod db;
mod db_extra;
mod error;
mod extract;
mod models;
mod routes;
mod tauri;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    let server = tokio::spawn(async { routes::run_server("0.0.0.0:3000").await });
    tokio::join!(server).0.expect("Failed to sstart server");
}

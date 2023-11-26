mod routes;
mod tauri;

#[tokio::main]
async fn main() {
    tokio::spawn(async { routes::run_server().await });
}

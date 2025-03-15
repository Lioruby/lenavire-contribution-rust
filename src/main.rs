use dotenv::dotenv;
use env_logger::Env;
use lenavire_contribution_rust::ledger::infrastructure::web::start_web_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    start_web_server().await
}

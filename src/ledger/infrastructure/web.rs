use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use log::info;

use crate::ledger::{
    infrastructure::adapters::websocket_event_stream::WebSocketEventStream, presentation::routes,
};

use super::adapters::{
    postgre_expense_repository::PostgreExpenseRepository,
    postgre_payment_repository::PostgrePaymentRepository, real_date_provider::RealDateProvider,
    real_id_provider::RealIdProvider,
};

pub async fn start_web_server() -> std::io::Result<()> {
    let id_provider = RealIdProvider::new();
    let date_provider = RealDateProvider::new();
    let payment_repository = PostgrePaymentRepository::new();
    let expense_repository = PostgreExpenseRepository::new();
    let event_stream = WebSocketEventStream::new();

    let app_data_id_provider = web::Data::new(id_provider);
    let app_data_date_provider = web::Data::new(date_provider);
    let app_data_payment_repository = web::Data::new(payment_repository);
    let app_data_expense_repository = web::Data::new(expense_repository);
    let app_data_event_stream = web::Data::new(event_stream);
    info!("Starting web server...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(app_data_id_provider.clone())
            .app_data(app_data_date_provider.clone())
            .app_data(app_data_payment_repository.clone())
            .app_data(app_data_expense_repository.clone())
            .app_data(app_data_event_stream.clone())
            .wrap(cors)
            .wrap(Logger::default())
            // .route("/ws", web::get().to(websocket_route))
            .configure(routes::payment_routes::routes)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}

use actix_web::{
    middleware::Logger,
    web::{self, route},
    App, HttpServer,
};
use log::info;

use crate::ledger::presentation::routes;

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

    let app_data_id_provider = web::Data::new(id_provider);
    let app_data_date_provider = web::Data::new(date_provider);
    let app_data_payment_repository = web::Data::new(payment_repository);
    let app_data_expense_repository = web::Data::new(expense_repository);

    info!("Starting web server...");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data_id_provider.clone())
            .app_data(app_data_date_provider.clone())
            .app_data(app_data_payment_repository.clone())
            .app_data(app_data_expense_repository.clone())
            .wrap(Logger::default())
            .configure(routes::payment_routes::routes)
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .await
}

use actix_web::web;

use crate::ledger::presentation::controllers::payments_controller::{
    add_expense, get_expenses_data, receive_payment,
};

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/ledger")
            .service(receive_payment)
            .service(add_expense)
            .service(get_expenses_data),
    );
}

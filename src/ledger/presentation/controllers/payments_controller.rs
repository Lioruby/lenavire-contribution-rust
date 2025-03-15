use crate::ledger::{
    application::{
        commands::{
            add_expense::handler::{AddExpenseCommand, AddExpenseHandler},
            receive_payment::handler::{ReceivePaymentCommand, ReceivePaymentHandler},
        },
        queries::get_expenses_data_query::{GetExpensesDataQuery, GetExpensesDataQueryHandler},
    },
    infrastructure::adapters::{
        postgre_expense_repository::PostgreExpenseRepository,
        postgre_payment_repository::PostgrePaymentRepository, real_date_provider::RealDateProvider,
        real_id_provider::RealIdProvider, websocket_event_stream::WebSocketEventStream,
    },
};
use actix_web::{get, post, web, HttpResponse};

use super::{add_expense_body, payment_received_body};

#[post("/payments-received")]
pub async fn receive_payment(
    payment_repository: web::Data<PostgrePaymentRepository>,
    id_provider: web::Data<RealIdProvider>,
    date_provider: web::Data<RealDateProvider>,
    expense_repository: web::Data<PostgreExpenseRepository>,
    event_stream: web::Data<WebSocketEventStream>,
    body: web::Json<payment_received_body::Request>,
) -> HttpResponse {
    let body = body.into_inner();

    let payment_type = if body.data.object.custom_fields[1]
        .dropdown
        .as_ref()
        .unwrap()
        .value
        == "ponctuel"
    {
        "one-time".to_string()
    } else {
        "recurring".to_string()
    };

    let command = ReceivePaymentCommand {
        amount: body.data.object.amount_total as f64,
        name: body.data.object.customer_details.name,
        email: body.data.object.customer_details.email,
        payment_type: payment_type,
    };

    match ReceivePaymentHandler::new(
        payment_repository.get_ref().clone(),
        id_provider.get_ref().clone(),
        date_provider.get_ref().clone(),
        expense_repository.get_ref().clone(),
        event_stream.get_ref().clone(),
    )
    .execute(command)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/expenses")]
pub async fn add_expense(
    expense_repository: web::Data<PostgreExpenseRepository>,
    id_provider: web::Data<RealIdProvider>,
    date_provider: web::Data<RealDateProvider>,
    event_stream: web::Data<WebSocketEventStream>,
    body: web::Json<add_expense_body::Request>,
) -> HttpResponse {
    let body = body.into_inner();
    let command = AddExpenseCommand {
        amount: body.data.amount,
    };

    if body.data.operation_type == "income" {
        return HttpResponse::Ok().finish();
    }

    match AddExpenseHandler::new(
        expense_repository.get_ref().clone(),
        id_provider.get_ref().clone(),
        date_provider.get_ref().clone(),
        event_stream.get_ref().clone(),
    )
    .execute(command)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/expenses-data")]
pub async fn get_expenses_data() -> HttpResponse {
    let query = GetExpensesDataQuery {};

    match GetExpensesDataQueryHandler::new().execute(query).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

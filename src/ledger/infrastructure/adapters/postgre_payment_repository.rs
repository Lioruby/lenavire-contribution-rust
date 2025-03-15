use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    ledger::{
        application::ports::payment_repository::PaymentRepository,
        domain::entities::payment::Payment,
        infrastructure::db::{
            connection::{establish_connection, DBPool},
            postgre_payment::PostgrePayment,
        },
    },
    schema,
};

#[derive(Debug, Clone)]
pub struct PostgrePaymentRepository {
    pool: DBPool,
}

impl PostgrePaymentRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection(&database_url);
        PostgrePaymentRepository { pool: pool }
    }
}

#[async_trait]
impl PaymentRepository for PostgrePaymentRepository {
    async fn create(&self, payment: Payment) -> () {
        let now = chrono::Utc::now().naive_utc();
        let payment_date = NaiveDateTime::parse_from_str(&payment.date, "%Y-%m-%d").unwrap_or(now);

        let postgre_payment = PostgrePayment {
            id: payment.id,
            amount: payment.amount.value as i64,
            name: payment.name,
            email: payment.email,
            payment_type: payment.payment_type,
            date: payment_date,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(schema::payments::table)
            .values(postgre_payment)
            .execute(&mut self.pool.get().unwrap())
            .unwrap();
    }
}

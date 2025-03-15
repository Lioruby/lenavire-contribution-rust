use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ledger::infrastructure::db::connection::{establish_connection, DBPool};
use crate::ledger::infrastructure::db::postgre_expense::PostgreExpense;
use crate::ledger::infrastructure::db::postgre_payment::PostgrePayment;
use crate::schema;

#[derive(Debug)]
pub struct GetExpensesDataQuery;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetExpensesDataQueryResponse {
    total_revenue: i64,
    total_expenses: i64,
    total_received: i64,
    payments: Vec<Payment>,
    top_contributors: Vec<TopContributor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    amount: i64,
    name: String,
    email: String,
    payment_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopContributor {
    amount: i64,
    name: String,
}

pub struct GetExpensesDataQueryHandler {
    db_pool: DBPool,
}

impl GetExpensesDataQueryHandler {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection(&database_url);
        Self { db_pool: pool }
    }

    pub async fn execute(
        &self,
        _query: GetExpensesDataQuery,
    ) -> Result<GetExpensesDataQueryResponse, diesel::result::Error> {
        let mut conn = self.db_pool.get().expect("DB connection failed");

        let all_expenses = schema::expenses::table
            .load::<PostgreExpense>(&mut conn)
            .expect("Error loading expenses");

        let all_payments = schema::payments::table
            .load::<PostgrePayment>(&mut conn)
            .expect("Error loading payments");

        let last_3_payments = schema::payments::table
            .order(schema::payments::date.desc())
            .limit(3)
            .load::<PostgrePayment>(&mut conn)
            .expect("Error loading payments");

        let mut top_contributors: Vec<TopContributor> = vec![];

        for payment in all_payments.clone() {
            if top_contributors.iter().any(|c| c.name == payment.name) {
                continue;
            }

            let all_person_payments = all_payments
                .iter()
                .filter(|p| p.email == payment.email)
                .map(|p| p.amount)
                .sum::<i64>();

            top_contributors.push(TopContributor {
                amount: all_person_payments,
                name: payment.name.clone(),
            });
        }

        Ok(GetExpensesDataQueryResponse {
            total_revenue: all_payments.iter().map(|p| p.amount).sum(),
            total_expenses: all_expenses.iter().map(|e| e.amount).sum(),
            total_received: all_payments.iter().map(|p| p.amount).sum(),
            payments: last_3_payments
                .iter()
                .map(|p| Payment {
                    amount: p.amount,
                    name: p.name.clone(),
                    email: p.email.clone(),
                    payment_type: p.payment_type.clone(),
                })
                .collect(),
            top_contributors: top_contributors,
        })
    }
}

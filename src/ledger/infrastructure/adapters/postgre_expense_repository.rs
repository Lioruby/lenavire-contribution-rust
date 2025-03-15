use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    ledger::{
        application::ports::expense_repository::ExpenseRepository,
        domain::entities::expense::Expense,
        infrastructure::db::{
            connection::{establish_connection, DBPool},
            postgre_expense::PostgreExpense,
        },
    },
    schema,
};

#[derive(Debug, Clone)]

pub struct PostgreExpenseRepository {
    pool: DBPool,
}

impl PostgreExpenseRepository {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = establish_connection(&database_url);
        PostgreExpenseRepository { pool: pool }
    }
}

#[async_trait]
impl ExpenseRepository for PostgreExpenseRepository {
    async fn create(&self, expense: Expense) -> () {
        let now = chrono::Utc::now().naive_utc();
        let postgre_expense = PostgreExpense {
            id: expense.id,
            amount: expense.amount.value,
            date: NaiveDateTime::parse_from_str(&expense.date, "%Y-%m-%d").unwrap_or(now),
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(schema::expenses::table)
            .values(postgre_expense)
            .execute(&mut self.pool.get().unwrap())
            .unwrap();
    }
}

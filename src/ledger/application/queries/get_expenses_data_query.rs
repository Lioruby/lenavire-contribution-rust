use async_trait::async_trait;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Jsonb, Nullable, Text};
use serde::{Deserialize, Serialize};

use crate::ledger::infrastructure::db::connection::{establish_connection, DBPool};

#[derive(Debug, QueryableByName, Queryable, Deserialize)]
struct SqlResult {
    #[diesel(sql_type = BigInt)]
    total_expenses: i64,

    #[diesel(sql_type = BigInt)]
    total_received: i64,

    #[diesel(sql_type = Nullable<Text>)]
    payments: Option<String>,

    #[diesel(sql_type = Nullable<Text>)]
    top_contributors: Option<String>,
}
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

        let query = r#"
            WITH payment_stats AS (
                SELECT 
                    COALESCE(SUM(amount), 0) AS total_received,
                    (
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'amount', amount,
                                'name', name,
                                'email', email,
                                'payment_type', "payment_type"
                            )
                        )
                        FROM (
                            SELECT * FROM "payments"
                            ORDER BY date DESC
                            LIMIT 3
                        ) recent
                    ) AS payments,
                    (
                        SELECT jsonb_agg(
                            jsonb_build_object(
                                'amount', total_amount,
                                'name', name
                            )
                        )
                        FROM (
                            SELECT 
                                email,
                                SUM(amount) AS total_amount,
                                MAX(name) AS name
                            FROM "payments"
                            WHERE date_trunc('month', date) = date_trunc('month', CURRENT_DATE)
                            GROUP BY email
                            ORDER BY total_amount DESC
                            LIMIT 20
                        ) top
                    ) AS top_contributors
                FROM "payments"
            ),
            expense_stats AS (
                SELECT COALESCE(SUM(amount), 0) AS total_expenses
                FROM "expenses"
            )
            SELECT 
                e.total_expenses,
                p.total_received,
                p.payments,
                p.top_contributors
            FROM payment_stats p
            CROSS JOIN expense_stats e
        "#;

        let result: SqlResult = diesel::sql_query(query).get_result(&mut conn)?;

        let payments: Vec<Payment> = result
            .payments
            .map(|json_str| serde_json::from_str(&json_str).unwrap_or_else(|_| vec![]))
            .unwrap_or_else(|| vec![]);

        let top_contributors: Vec<TopContributor> = result
            .top_contributors
            .map(|json_str| serde_json::from_str(&json_str).unwrap_or_else(|_| vec![]))
            .unwrap_or_else(|| vec![]);

        Ok(GetExpensesDataQueryResponse {
            total_revenue: result.total_received - result.total_expenses,
            total_expenses: result.total_expenses,
            total_received: result.total_received,
            payments,
            top_contributors,
        })
    }
}

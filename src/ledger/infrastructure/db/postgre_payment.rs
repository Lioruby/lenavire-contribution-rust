use crate::schema::payments;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = payments)]
pub struct PostgrePayment {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub amount: i64,
    pub name: String,
    pub email: String,
    pub payment_type: String,
    pub date: chrono::NaiveDateTime,
}

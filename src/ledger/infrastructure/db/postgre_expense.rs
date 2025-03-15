use crate::schema::expenses;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(table_name = expenses)]
pub struct PostgreExpense {
    pub id: String,
    pub amount: f64,
    pub date: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

use crate::ledger::domain::entities::expense::Expense;
use async_trait::async_trait;

#[async_trait]
pub trait ExpenseRepository {
    async fn create(&self, expense: Expense) -> ();
}

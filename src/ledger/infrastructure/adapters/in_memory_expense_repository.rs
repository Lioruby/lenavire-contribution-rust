use crate::ledger::application::ports::expense_repository::ExpenseRepository;
use crate::ledger::domain::entities::expense::Expense;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default, Clone)]
pub struct InMemoryExpenseRepository {
    pub expenses: Arc<Mutex<Vec<Expense>>>,
}

impl InMemoryExpenseRepository {
    pub fn new() -> Self {
        Self {
            expenses: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl ExpenseRepository for InMemoryExpenseRepository {
    async fn create(&self, expense: Expense) {
        let mut expenses = self.expenses.lock().unwrap();
        expenses.push(expense);
    }
}

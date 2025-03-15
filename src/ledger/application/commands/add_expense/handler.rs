use crate::ledger::application::ports::date_provider::DateProvider;
use crate::ledger::application::ports::expense_repository::ExpenseRepository;
use crate::ledger::application::ports::id_provider::IdProvider;
use crate::ledger::domain::entities::expense::Expense;
use crate::ledger::domain::entities::expense::ExpenseProps;
use crate::ledger::domain::value_objects::amount::Amount;

#[derive(Debug)]
pub struct AddExpenseCommand {
    pub amount: Amount,
}

pub struct AddExpenseHandler {
    pub repository: Box<dyn ExpenseRepository>,
    pub id_provider: Box<dyn IdProvider>,
    pub date_provider: Box<dyn DateProvider>,
}

impl AddExpenseHandler {
    pub fn new(
        repository: Box<dyn ExpenseRepository>,
        id_provider: Box<dyn IdProvider>,
        date_provider: Box<dyn DateProvider>,
    ) -> Self {
        Self {
            repository,
            id_provider,
            date_provider,
        }
    }

    pub async fn execute(&self, command: AddExpenseCommand) -> Result<(), String> {
        let expense = Expense::new(ExpenseProps {
            amount: command.amount,
            date: self.date_provider.now(),
            id: self.id_provider.generate(),
        });
        self.repository.create(expense).await;
        Ok(())
    }
}

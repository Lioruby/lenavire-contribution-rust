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

pub struct AddExpenseHandler<R: ExpenseRepository, I: IdProvider, D: DateProvider> {
    pub repository: R,
    pub id_provider: I,
    pub date_provider: D,
}

impl<R: ExpenseRepository, I: IdProvider, D: DateProvider> AddExpenseHandler<R, I, D> {
    pub fn new(repository: R, id_provider: I, date_provider: D) -> Self {
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

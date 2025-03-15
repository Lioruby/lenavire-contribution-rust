use crate::ledger::application::ports::date_provider::DateProvider;
use crate::ledger::application::ports::event_streams::EventStream;
use crate::ledger::application::ports::expense_repository::ExpenseRepository;
use crate::ledger::application::ports::id_provider::IdProvider;
use crate::ledger::domain::entities::expense::Expense;
use crate::ledger::domain::entities::expense::ExpenseProps;
use crate::ledger::domain::value_objects::amount::Amount;

#[derive(Debug)]
pub struct AddExpenseCommand {
    pub amount: f64,
}

pub struct AddExpenseHandler<R: ExpenseRepository, I: IdProvider, D: DateProvider, ES: EventStream>
{
    pub repository: R,
    pub id_provider: I,
    pub date_provider: D,
    pub event_stream: ES,
}

impl<R: ExpenseRepository, I: IdProvider, D: DateProvider, ES: EventStream>
    AddExpenseHandler<R, I, D, ES>
{
    pub fn new(repository: R, id_provider: I, date_provider: D, event_stream: ES) -> Self {
        Self {
            repository,
            id_provider,
            date_provider,
            event_stream,
        }
    }

    pub async fn execute(&self, command: AddExpenseCommand) -> Result<(), String> {
        let expense = Expense::new(ExpenseProps {
            amount: Amount::new(command.amount)?,
            date: self.date_provider.now(),
            id: self.id_provider.generate(),
        });
        self.repository.create(expense).await;
        self.event_stream.send("expense-added").await;
        Ok(())
    }
}

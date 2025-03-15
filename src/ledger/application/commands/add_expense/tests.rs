#[cfg(test)]
mod tests {
    use crate::ledger::application::commands::add_expense::handler::AddExpenseCommand;
    use crate::ledger::application::commands::add_expense::handler::AddExpenseHandler;
    use crate::ledger::infrastructure::adapters::fixed_date_provider::FixedDateProvider;
    use crate::ledger::infrastructure::adapters::fixed_id_provider::FixedIdProvider;
    use crate::ledger::infrastructure::adapters::in_memory_expense_repository::InMemoryExpenseRepository;

    #[tokio::test]
    async fn it_should_create_an_expense() {
        let expense_repository = InMemoryExpenseRepository::new();
        let repo_clone = expense_repository.clone();
        let handler = AddExpenseHandler::new(
            expense_repository,
            FixedIdProvider::default(),
            FixedDateProvider::default(),
        );

        let command = AddExpenseCommand { amount: 100 as f64 };

        handler.execute(command).await.unwrap();

        let expenses = repo_clone.expenses.lock().unwrap();
        assert_eq!(expenses.len(), 1);
    }

    #[tokio::test]
    async fn it_should_generate_an_id() {
        let expense_repository = InMemoryExpenseRepository::new();
        let repo_clone = expense_repository.clone();
        let handler = AddExpenseHandler::new(
            expense_repository,
            FixedIdProvider::default(),
            FixedDateProvider::default(),
        );

        let command = AddExpenseCommand { amount: 100 as f64 };

        handler.execute(command).await.unwrap();

        let expenses = repo_clone.expenses.lock().unwrap();

        assert_eq!(expenses[0].id, "123");
    }

    #[tokio::test]
    async fn it_should_mark_the_date_of_the_expense() {
        let expense_repository = InMemoryExpenseRepository::new();
        let date_provider = FixedDateProvider::default();
        let repo_clone = expense_repository.clone();
        let handler = AddExpenseHandler::new(
            expense_repository,
            FixedIdProvider::default(),
            date_provider,
        );

        let command = AddExpenseCommand { amount: 100 as f64 };

        handler.execute(command).await.unwrap();

        let expenses = repo_clone.expenses.lock().unwrap();

        assert_eq!(expenses[0].date, "2021-01-01");
    }
}

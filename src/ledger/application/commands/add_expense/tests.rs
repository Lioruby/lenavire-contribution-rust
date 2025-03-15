#[cfg(test)]
mod tests {
    use crate::ledger::application::commands::add_expense::handler::{
        AddExpenseCommand, AddExpenseHandler,
    };
    use crate::ledger::application::ports::date_provider::DateProvider;
    use crate::ledger::application::ports::id_provider::IdProvider;
    use crate::ledger::infrastructure::adapters::fixed_date_provider::FixedDateProvider;
    use crate::ledger::infrastructure::adapters::fixed_id_provider::FixedIdProvider;
    use crate::ledger::infrastructure::adapters::in_memory_event_stream::InMemoryEventStream;
    use crate::ledger::infrastructure::adapters::in_memory_expense_repository::InMemoryExpenseRepository;

    struct TestContext {
        expense_repository: InMemoryExpenseRepository,
        id_provider: FixedIdProvider,
        date_provider: FixedDateProvider,
        event_stream: InMemoryEventStream,
        handler: AddExpenseHandler<
            InMemoryExpenseRepository,
            FixedIdProvider,
            FixedDateProvider,
            InMemoryEventStream,
        >,
    }

    impl TestContext {
        fn new() -> Self {
            let expense_repository = InMemoryExpenseRepository::new();
            let id_provider = FixedIdProvider::new();
            let date_provider = FixedDateProvider::new();
            let event_stream = InMemoryEventStream::new();
            let handler = AddExpenseHandler::new(
                expense_repository.clone(),
                id_provider.clone(),
                date_provider.clone(),
                event_stream.clone(),
            );

            TestContext {
                expense_repository,
                id_provider,
                date_provider,
                event_stream,
                handler,
            }
        }

        fn create_command(&self) -> AddExpenseCommand {
            AddExpenseCommand { amount: 100 as f64 }
        }
    }

    #[tokio::test]
    async fn it_should_create_an_expense() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        ctx.handler.execute(command).await.unwrap();

        let expenses = ctx.expense_repository.expenses.lock().unwrap();
        assert_eq!(expenses.len(), 1);
    }

    #[tokio::test]
    async fn it_should_generate_an_id() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        ctx.handler.execute(command).await.unwrap();

        let expenses = ctx.expense_repository.expenses.lock().unwrap();
        assert_eq!(expenses[0].id, ctx.id_provider.generate());
    }

    #[tokio::test]
    async fn it_should_mark_the_date_of_the_expense() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        ctx.handler.execute(command).await.unwrap();

        let expenses = ctx.expense_repository.expenses.lock().unwrap();
        assert_eq!(expenses[0].date, ctx.date_provider.now());
    }

    #[tokio::test]
    async fn it_should_reject_if_the_amount_is_negative() {
        let ctx = TestContext::new();
        let command = AddExpenseCommand {
            amount: -100 as f64,
        };

        let result = ctx.handler.execute(command).await;
        assert_eq!(result, Err("Amount must be positive".to_string()));
    }

    #[tokio::test]
    async fn it_should_send_an_event_to_the_ledger_activity_stream() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        ctx.handler.execute(command).await.unwrap();

        let events = ctx.event_stream.events.lock().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "expense-added");
    }
}

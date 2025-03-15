mod tests {
    use crate::ledger::application::commands::receive_payment::handler::{
        ReceivePaymentCommand, ReceivePaymentHandler,
    };
    use crate::ledger::application::ports::date_provider::DateProvider;
    use crate::ledger::application::ports::id_provider::IdProvider;
    use crate::ledger::infrastructure::adapters::fixed_date_provider::FixedDateProvider;
    use crate::ledger::infrastructure::adapters::fixed_id_provider::FixedIdProvider;
    use crate::ledger::infrastructure::adapters::in_memory_event_stream::InMemoryEventStream;
    use crate::ledger::infrastructure::adapters::in_memory_expense_repository::InMemoryExpenseRepository;
    use crate::ledger::infrastructure::adapters::in_memory_payment_repository::InMemoryPaymentRepository;

    struct TestContext {
        payment_repository: InMemoryPaymentRepository,
        expense_repository: InMemoryExpenseRepository,
        id_provider: FixedIdProvider,
        date_provider: FixedDateProvider,
        event_stream: InMemoryEventStream,
        handler: ReceivePaymentHandler<
            InMemoryPaymentRepository,
            FixedIdProvider,
            FixedDateProvider,
            InMemoryExpenseRepository,
            InMemoryEventStream,
        >,
    }

    impl TestContext {
        fn new() -> Self {
            let payment_repository = InMemoryPaymentRepository::new();
            let expense_repository = InMemoryExpenseRepository::new();
            let id_provider = FixedIdProvider::new();
            let date_provider = FixedDateProvider::new();
            let event_stream = InMemoryEventStream::new();
            let handler = ReceivePaymentHandler::new(
                payment_repository.clone(),
                id_provider.clone(),
                date_provider.clone(),
                expense_repository.clone(),
                event_stream.clone(),
            );

            TestContext {
                payment_repository,
                expense_repository,
                id_provider,
                date_provider,
                event_stream,
                handler,
            }
        }

        fn create_command(&self) -> ReceivePaymentCommand {
            ReceivePaymentCommand {
                amount: 100 as f64,
                name: "John Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                payment_type: "recurring".to_string(),
            }
        }
    }

    #[tokio::test]
    async fn it_should_create_a_payment() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        let _ = ctx.handler.execute(command).await;

        let payments = ctx.payment_repository.payments.lock().unwrap();
        assert_eq!(payments.len(), 1);
    }

    #[tokio::test]
    async fn it_should_generate_an_id() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        let _ = ctx.handler.execute(command).await;

        let payments = ctx.payment_repository.payments.lock().unwrap();
        assert_eq!(payments[0].id, ctx.id_provider.generate());
    }

    #[tokio::test]
    async fn it_should_mark_the_date_of_the_payment() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        let _ = ctx.handler.execute(command).await;

        let payments = ctx.payment_repository.payments.lock().unwrap();
        assert_eq!(payments[0].date, ctx.date_provider.now());
    }

    #[tokio::test]
    async fn it_should_create_an_expense_of_20_percent_of_the_payment() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        let _ = ctx.handler.execute(command).await;

        let expenses = ctx.expense_repository.expenses.lock().unwrap();
        assert_eq!(expenses[0].amount.value, 20.0);
    }

    #[tokio::test]
    async fn it_should_reject_if_the_amount_is_negative() {
        let ctx = TestContext::new();
        let command = ReceivePaymentCommand {
            amount: -100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        let result = ctx.handler.execute(command).await;
        assert_eq!(result, Err("Amount must be positive".to_string()));
    }

    #[tokio::test]
    async fn it_should_send_an_event_to_the_ledger_activity_stream() {
        let ctx = TestContext::new();
        let command = ctx.create_command();

        let _ = ctx.handler.execute(command).await;

        let events = ctx.event_stream.events;
        assert_eq!(events.lock().await.len(), 1);
    }
}

mod tests {
    use crate::ledger::application::commands::receive_payment::handler::{
        ReceivePaymentCommand, ReceivePaymentHandler,
    };
    use crate::ledger::infrastructure::adapters::fixed_date_provider::FixedDateProvider;
    use crate::ledger::infrastructure::adapters::fixed_id_provider::FixedIdProvider;
    use crate::ledger::infrastructure::adapters::in_memory_expense_repository::InMemoryExpenseRepository;
    use crate::ledger::infrastructure::adapters::in_memory_payment_repository::InMemoryPaymentRepository;

    #[tokio::test]
    async fn it_should_create_a_payment() {
        let payment_repository = InMemoryPaymentRepository::new();
        let id_provider = FixedIdProvider::new();
        let date_provider = FixedDateProvider::new();
        let repo_clone = payment_repository.clone();
        let expense_repository = InMemoryExpenseRepository::new();
        let handler = ReceivePaymentHandler::new(
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        );

        let command = ReceivePaymentCommand {
            amount: 100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        handler.execute(command).await.unwrap();

        let payments = repo_clone.payments.lock().unwrap();
        assert_eq!(payments.len(), 1);
    }

    #[tokio::test]
    async fn it_should_generate_an_id() {
        let payment_repository = InMemoryPaymentRepository::new();
        let date_provider = FixedDateProvider::new();
        let repo_clone = payment_repository.clone();
        let id_provider = FixedIdProvider::new();
        let expense_repository = InMemoryExpenseRepository::new();
        let handler = ReceivePaymentHandler::new(
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        );

        let command = ReceivePaymentCommand {
            amount: 100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        handler.execute(command).await.unwrap();

        let payments = repo_clone.payments.lock().unwrap();
        assert_eq!(payments[0].id, "123");
    }

    #[tokio::test]
    async fn it_should_mark_the_date_of_the_payment() {
        let payment_repository = InMemoryPaymentRepository::new();
        let repo_clone = payment_repository.clone();
        let expense_repository = InMemoryExpenseRepository::new();
        let id_provider = FixedIdProvider::new();
        let date_provider = FixedDateProvider::new();
        let handler = ReceivePaymentHandler::new(
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        );

        let command = ReceivePaymentCommand {
            amount: 100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        handler.execute(command).await.unwrap();

        let payments = repo_clone.payments.lock().unwrap();
        assert_eq!(payments[0].date, "2021-01-01");
    }

    #[tokio::test]
    async fn it_should_create_an_expense_of_20_percent_of_the_payment() {
        let payment_repository = InMemoryPaymentRepository::new();
        let expense_repository = InMemoryExpenseRepository::new();
        let repo_clone = expense_repository.clone();
        let id_provider = FixedIdProvider::new();
        let date_provider = FixedDateProvider::new();
        let handler = ReceivePaymentHandler::new(
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        );

        let command = ReceivePaymentCommand {
            amount: 100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        handler.execute(command).await.unwrap();

        let expenses = repo_clone.expenses.lock().unwrap();
        assert_eq!(expenses[0].amount.value, 20.0);
    }

    #[tokio::test]
    async fn it_should_reject_if_the_amount_is_negative() {
        let payment_repository = InMemoryPaymentRepository::new();
        let expense_repository = InMemoryExpenseRepository::new();
        let id_provider = FixedIdProvider::new();
        let date_provider = FixedDateProvider::new();
        let handler = ReceivePaymentHandler::new(
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        );

        let command = ReceivePaymentCommand {
            amount: -100 as f64,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            payment_type: "recurring".to_string(),
        };

        let result = handler.execute(command).await;
        assert_eq!(result, Err("Amount must be positive".to_string()));
    }
}

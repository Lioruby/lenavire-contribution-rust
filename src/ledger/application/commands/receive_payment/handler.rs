use crate::ledger::{
    application::ports::{
        date_provider::DateProvider, expense_repository::ExpenseRepository,
        id_provider::IdProvider, payment_repository::PaymentRepository,
    },
    domain::{
        entities::{
            expense::{Expense, ExpenseProps},
            payment::{Payment, PaymentProps},
        },
        value_objects::amount::Amount,
    },
};

#[derive(Debug)]
pub struct ReceivePaymentCommand {
    pub amount: f64,
    pub name: String,
    pub email: String,
    pub payment_type: String,
}

pub struct ReceivePaymentHandler<
    R: PaymentRepository,
    I: IdProvider,
    D: DateProvider,
    E: ExpenseRepository,
> {
    pub payment_repository: R,
    pub id_provider: I,
    pub date_provider: D,
    pub expense_repository: E,
}

impl<R: PaymentRepository, I: IdProvider, D: DateProvider, E: ExpenseRepository>
    ReceivePaymentHandler<R, I, D, E>
{
    const TVA_RATE: f64 = 0.2;

    pub fn new(
        payment_repository: R,
        id_provider: I,
        date_provider: D,
        expense_repository: E,
    ) -> Self {
        Self {
            payment_repository,
            id_provider,
            date_provider,
            expense_repository,
        }
    }

    pub async fn execute(&self, command: ReceivePaymentCommand) -> Result<(), String> {
        let payment = self.create_payment(&command)?;
        self.payment_repository.create(payment).await;

        let expense = self.create_default_tva_expense(&command)?;
        self.expense_repository.create(expense).await;
        Ok(())
    }

    fn create_payment(&self, command: &ReceivePaymentCommand) -> Result<Payment, String> {
        Ok(Payment::new(PaymentProps {
            id: self.id_provider.generate(),
            amount: Amount::new(command.amount)?,
            name: command.name.clone(),
            email: command.email.clone(),
            payment_type: command.payment_type.clone(),
            date: self.date_provider.now(),
        }))
    }

    fn create_default_tva_expense(
        &self,
        command: &ReceivePaymentCommand,
    ) -> Result<Expense, String> {
        Ok(Expense::new(ExpenseProps {
            id: self.id_provider.generate(),
            amount: Amount::new(command.amount * Self::TVA_RATE)?,
            date: self.date_provider.now(),
        }))
    }
}

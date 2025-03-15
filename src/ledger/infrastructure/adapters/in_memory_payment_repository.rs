use crate::ledger::application::ports::payment_repository::PaymentRepository;
use crate::ledger::domain::entities::payment::Payment;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default, Clone)]
pub struct InMemoryPaymentRepository {
    pub payments: Arc<Mutex<Vec<Payment>>>,
}

impl InMemoryPaymentRepository {
    pub fn new() -> Self {
        Self {
            payments: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl PaymentRepository for InMemoryPaymentRepository {
    async fn create(&self, payment: Payment) -> () {
        let mut payments = self.payments.lock().unwrap();
        payments.push(payment);
    }
}

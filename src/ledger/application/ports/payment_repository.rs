use async_trait::async_trait;

use crate::ledger::domain::entities::payment::Payment;

#[async_trait]
pub trait PaymentRepository {
    async fn create(&self, payment: Payment) -> ();
}

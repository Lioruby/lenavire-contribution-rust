use crate::ledger::application::ports::id_provider::IdProvider;
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct RealIdProvider {}

impl RealIdProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdProvider for RealIdProvider {
    fn generate(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

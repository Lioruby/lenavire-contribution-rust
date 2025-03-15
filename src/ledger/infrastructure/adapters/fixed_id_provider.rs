use crate::ledger::application::ports::id_provider::IdProvider;

#[derive(Default, Clone)]

pub struct FixedIdProvider {}

impl FixedIdProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl IdProvider for FixedIdProvider {
    fn generate(&self) -> String {
        "123".to_string()
    }
}

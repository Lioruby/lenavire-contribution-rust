use crate::ledger::application::ports::id_provider::IdProvider;

#[derive(Default)]

pub struct FixedIdProvider {}

impl IdProvider for FixedIdProvider {
    fn generate(&self) -> String {
        "123".to_string()
    }
}

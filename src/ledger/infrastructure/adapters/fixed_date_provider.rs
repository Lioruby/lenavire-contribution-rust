use crate::ledger::application::ports::date_provider::DateProvider;

#[derive(Default, Clone)]
pub struct FixedDateProvider {}

impl FixedDateProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl DateProvider for FixedDateProvider {
    fn now(&self) -> String {
        "2021-01-01".to_string()
    }
}

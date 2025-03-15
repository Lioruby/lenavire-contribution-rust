use crate::ledger::application::ports::date_provider::DateProvider;

#[derive(Default)]
pub struct FixedDateProvider {}

impl DateProvider for FixedDateProvider {
    fn now(&self) -> String {
        "2021-01-01".to_string()
    }
}

use crate::ledger::application::ports::date_provider::DateProvider;

#[derive(Default, Clone)]
pub struct RealDateProvider {}

impl RealDateProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl DateProvider for RealDateProvider {
    fn now(&self) -> String {
        chrono::Utc::now().to_rfc3339()
    }
}

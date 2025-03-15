#[derive(Debug)]
pub struct Amount {
    pub value: f64,
}

impl Amount {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

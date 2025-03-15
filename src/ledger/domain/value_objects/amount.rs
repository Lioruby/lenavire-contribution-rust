#[derive(Debug)]
pub struct Amount {
    pub value: f64,
}

impl Amount {
    pub fn new(value: f64) -> Result<Self, String> {
        let amount = Self { value };
        amount.validate()?;
        Ok(amount)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.value < 0.0 {
            Err("Amount must be positive".to_string())
        } else {
            Ok(())
        }
    }
}

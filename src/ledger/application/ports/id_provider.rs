pub trait IdProvider {
    fn generate(&self) -> String;
}

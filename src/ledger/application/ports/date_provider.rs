pub trait DateProvider {
    fn now(&self) -> String;
}

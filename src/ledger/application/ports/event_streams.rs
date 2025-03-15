use async_trait::async_trait;

#[async_trait]
pub trait EventStream {
    async fn send(&self, event: &str);
}

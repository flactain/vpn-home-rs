use async_trait::async_trait;

#[async_trait]
pub trait ServersRepository: Send+Sync {
    async fn find_all(&self);
}

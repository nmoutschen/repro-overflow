use crate::file_1::MyClient;

#[async_trait::async_trait]
pub trait Get {
    async fn get(&self, key: &str) -> Option<String>;
}

#[async_trait::async_trait]
impl Get for MyClient {
    async fn get(&self, key: &str) -> Option<String> {
        self.get(key).await
    }
}

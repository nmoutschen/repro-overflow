use crate::file_1::MyClient;

#[async_trait::async_trait]
pub trait ClientTrait {
    fn get(&self, key: &str) -> Option<String>;
    fn put(&mut self, key: &str, value: String);
}

#[async_trait::async_trait]
impl ClientTrait for MyClient {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn put(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }
}

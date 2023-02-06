use repro_overflow::file_1::MyClient;
use repro_overflow::file_3::Get;

#[tokio::main]
async fn main() {
    let client = MyClient {
        data: Default::default(),
    };

    client.get("abc").await;
}

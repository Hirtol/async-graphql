use std::time::Duration;

use reqwest::Client;

pub fn client() -> Client {
    Client::builder().no_proxy().build().unwrap()
}

pub async fn wait_server_ready() {
    async_std::task::sleep(Duration::from_secs(1)).await;
}

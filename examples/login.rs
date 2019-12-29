mod api;

#[tokio::main]
async fn main() {
    let mut api = api::get_api();
    api.login().await.unwrap();
    api.store_token().unwrap();
}
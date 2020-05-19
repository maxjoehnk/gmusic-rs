use gmusic::{auth::stdio_login, GoogleMusicApi};

#[tokio::main]
async fn main() {
    env_logger::init();
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    let api = GoogleMusicApi::new(client_id, client_secret, None).unwrap();

    api.login(stdio_login).await.unwrap();
    api.store_token().await.unwrap();
}

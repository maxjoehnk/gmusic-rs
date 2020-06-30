use gmusic::GoogleMusicApi;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    let api = GoogleMusicApi::new(client_id, client_secret, None).unwrap();
    api.load_token().await.unwrap();

    let content = api.get_shared_playlist_contents("AMaBXynwNDXT76AoLfKqdCuzH_xy69khC3m8SrxN54zlxkNdbTuiv-fXyMpwfN8aS46J8kOy64KghXDVJychpr825G5dvUEB6Q==").await.unwrap();
    println!("{:#?}", content);
}

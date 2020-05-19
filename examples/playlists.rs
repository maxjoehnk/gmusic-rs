use gmusic::GoogleMusicApi;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    let api = GoogleMusicApi::new(client_id, client_secret, None).unwrap();
    api.load_token().await.unwrap();

    let playlists = api.get_all_playlists().await.unwrap();
    println!("{:#?}", playlists);
}

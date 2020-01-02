use gmusic::GoogleMusicApi;

fn main() {
    env_logger::init();
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    let api = GoogleMusicApi::new(client_id, client_secret).unwrap();
    api.load_token().unwrap();

    let playlists = api.get_playlist_entries().unwrap();
    println!("{:#?}", playlists);
}

use gmusic::GoogleMusicApi;

pub fn get_api() -> GoogleMusicApi {
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    GoogleMusicApi::new(client_id, client_secret)
}
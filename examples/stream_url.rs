use gmusic::GoogleMusicApi;

fn main() {
    env_logger::init();
    let client_id = std::env::var("CLIENT_ID").expect("missing client id");
    let client_secret = std::env::var("CLIENT_SECRET").expect("missing client secret");

    let api = GoogleMusicApi::new(client_id, client_secret).unwrap();
    api.load_token().unwrap();

    let tracks = api.get_all_tracks().unwrap();
    let track = &tracks[0];

    let device_id = std::env::var("DEVICE_ID").expect("missing device id");
    let url = api.get_stream_url(&track.id, &device_id).unwrap();
    println!("{}", url);
}

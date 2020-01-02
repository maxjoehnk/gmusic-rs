mod api;

fn main() {
    let mut api = api::get_api();
    api.load_token().unwrap();

    let tracks = api.get_all_tracks().unwrap();
    let track = &tracks[0];

    let device_id = std::env::var("DEVICE_ID").expect("missing device id");
    let url = api.get_stream_url(&track.id, &device_id).unwrap();
    println!("{}", url);
}

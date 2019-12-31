mod api;

fn main() {
    let mut api = api::get_api();
    api.load_token().unwrap();

    let playlists = api.get_all_playlists().unwrap();
    println!("{:#?}", playlists);
}
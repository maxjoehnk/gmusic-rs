mod api;

#[tokio::main]
async fn main() {
    let mut api = api::get_api();
    api.load_token().unwrap();

    let tracks = api.get_all_tracks().await.unwrap();
    println!("{:#?}", tracks);
}
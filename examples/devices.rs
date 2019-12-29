mod api;

#[tokio::main]
async fn main() {
    let mut api = api::get_api();
    api.load_token().unwrap();

    let devices = api.get_device_management_info().await.unwrap();
    println!("{:#?}", devices);
}
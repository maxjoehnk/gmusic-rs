mod api;

fn main() {
    let mut api = api::get_api();
    api.load_token().unwrap();

    let devices = api.get_device_management_info().unwrap();
    println!("{:#?}", devices);
}
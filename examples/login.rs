mod api;

fn main() {
    let mut api = api::get_api();
    api.login().unwrap();
    api.store_token().unwrap();
}
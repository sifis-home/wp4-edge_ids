pub mod network;

pub fn routes() -> Vec<rocket::Route> {
    network::routes()
}

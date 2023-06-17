use dotenv::dotenv;
use vina_sd::api::ApiClient;

fn main() {
    dotenv().ok();
    let api_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");
    let client = ApiClient::new(api_url);
    client.test().unwrap();
}

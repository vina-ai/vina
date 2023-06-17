use vina_story::api::ApiClient;

fn main() {
    let token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");
    let client = ApiClient::new(token);
    client.test().unwrap();
}

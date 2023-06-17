use std::path::PathBuf;

use vina_sd::api::ApiClient;

fn main() {
    let api_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");
    let client = ApiClient::new(api_url);

    client.request("1girl, bishoujo, casual, indoors, standing, 25 years old, fair skin, emerald-green eyes, wavy chestnut hair, cascading curls, vibrant and eclectic clothing, colorful dresses with delicate lace and intricate patterns, whimsical hats, mismatched socks, creative spirit, full body portrait, no background, anime art style.", &PathBuf::from("./output.png")).unwrap();
}

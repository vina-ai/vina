use clap::Parser;
use dotenvy::dotenv;
use vina_codegen::generate_proj;
use vina_sd::generate_character_art;
use vina_story::{content::*, generate_character_prompt, generate_story};

#[derive(Parser)]
struct Cli {
    name: String,
    prompt: String,
    out: std::path::PathBuf,
}
fn main() {
    // let args = Cli::parse();
    // dotenv().ok();

    let ren_path: String = std::env::var("REN_PATH").unwrap_or("renpy".to_string());
    let openai_token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");
    let novelai_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");

    let novelai_client = vina_sd::api::ApiClient::new(novelai_url);

    println!("Generating game...");
    let game = generate_story(&openai_token).unwrap();
    println!("{:?}", game);

    for character in game.characters {
        let character_description = generate_character_prompt(&openai_token, &character).unwrap();
        // println!("{character_description}");
        generate_character_art(&novelai_client, &character, &character_description);
    }

    /*

    match generate_proj(
        ren_path,
        args.name,
        args.prompt,
        args.out,
        scenes,
        characters,
    ) {
        Ok(()) => return,
        Err(error) => {
            println!("{:?}", error);
            return;
        },
    }
    */
}

use std::{fs::OpenOptions, io::Write, path::PathBuf};

use clap::Parser;
use dotenvy::dotenv;
use vina_sd::{generate_character_art, generate_location_art};
use vina_story::{content::*, generate_character_prompt, generate_location_prompt, generate_story};

mod codegen;
use codegen::generate_proj;

#[derive(Parser)]
struct Cli {
    // /// OVerride Name of the game
    // name: Option<String>,
    // prompt: String,
    // /// Override the output path of project
    // out: std::path::PathBuf,
    /// Save the generated game data to file
    #[arg(long, default_value_t = false)]
    save: bool,
}

fn main() {
    let args = Cli::parse();
    dotenv().ok();

    let ren_path: String = std::env::var("REN_PATH").unwrap_or("renpy".to_string());
    let openai_token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");
    let novelai_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");

    let novelai_client = vina_sd::api::ApiClient::new(novelai_url);

    println!("Generating game...");
    let prompt = "Write a love story about two visual novel developers.";
    let game = generate_story(&openai_token, prompt).unwrap();
    println!("{:?}", game);

    // Write completed game to file to be reloaded
    if args.save {
        let path = PathBuf::from("./game.ron");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let ron_encoded = ron::ser::to_string_pretty(&game, ron::ser::PrettyConfig::default())
            .expect("Error serializing game object");
        file.write_all(ron_encoded.as_bytes()).unwrap();
    }

    return;

    // Generate art for each character
    for character in game.characters.iter() {
        let character_description = generate_character_prompt(&openai_token, &character).unwrap();
        // println!("{character_description}");
        generate_character_art(&novelai_client, &character, &character_description);
    }

    // Generate art for each scene
    for (i, scene) in game.scenes.iter().enumerate() {
        let location_description =
            generate_location_prompt(&openai_token, &scene.location).unwrap();
        let location = generate_location_art(
            &novelai_client,
            i.to_string(),
            &scene.location,
            &location_description,
        );
    }

    // TODO can generate project name from prompt too
    // let project_name = args.name.unwrap_or(String::from("VINA game"));
    let project_path = PathBuf::from("./");
    generate_proj(&game, &project_path);
}

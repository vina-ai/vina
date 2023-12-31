use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use vina_sd::{generate_character_art, generate_location_art};
use vina_story::{
    content::*,
    generate_character_prompt, generate_location_prompt, generate_story,
    music::{choose_music, fetch_music},
};

mod codegen;
use codegen::generate_proj;

#[derive(Parser)]
struct Cli {
    prompt: String,
    // /// Override the output path of project
    out: std::path::PathBuf,
    /// Save the generated game data to file
    #[arg(long, default_value_t = false)]
    save: bool,
    /// Load game files from RON
    #[arg(long)]
    game_file: Option<String>,
    /// Skip generating characters
    #[arg(long, default_value_t = false)]
    skip_character: bool,
    /// Skip generating background art
    #[arg(long, default_value_t = false)]
    skip_background: bool,
    /// Skip generating music
    #[arg(long, default_value_t = false)]
    skip_music: bool,
}

fn main() {
    let args = Cli::parse();
    dotenv().ok();

    let ren_path: String = std::env::var("REN_PATH").unwrap_or("renpy".to_string());
    let openai_token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");
    let novelai_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");

    let novelai_client = vina_sd::api::ApiClient::new(novelai_url);

    let game = generate_game(&args, &openai_token);

    // Generate art for each character
    if !args.skip_character {
        for character in game.characters.iter() {
            let character_description =
                generate_character_prompt(&openai_token, &character).unwrap();
            // println!("{character_description}");
            let expressions = vec!["smiling", "crying", "nervous", "excited", "blushing"];
            generate_character_art(
                &novelai_client,
                &character,
                &character_description,
                expressions,
            );
        }
    }

    // Generate art for each scene
    for (i, scene) in game.scenes.iter().enumerate() {
        if !args.skip_background {
            let location_description =
                generate_location_prompt(&openai_token, &scene.location).unwrap();
            let location = generate_location_art(
                &novelai_client,
                i.to_string(),
                &scene.location,
                &location_description,
            );
        }

        // Generate music for the scene
        if !args.skip_music {
            let music_id = choose_music(scene.music);
            let music_path = PathBuf::from(format!("./music_{i}.mp3"));
            fetch_music(&music_id, &music_path).unwrap();
        }
    }

    // TODO can generate project name from prompt too
    // let project_name = args.name.unwrap_or(String::from("VINA game"));
    generate_proj(&game, &args.out).unwrap();
    run_game(&PathBuf::from(ren_path), &args.out, &game).unwrap();
}

fn generate_game(args: &Cli, openai_token: &str) -> Game {
    if let Some(game_file) = &args.game_file {
        println!("Loading game from file...");

        let path = PathBuf::from(game_file);
        let contents = fs::read_to_string(&path).unwrap();
        let game: Game = ron::from_str(&contents).unwrap();
        game
    } else {
        // Otherwise generate game from scratch

        println!("Generating game...");
        let game = generate_story(openai_token, args.prompt.as_str()).unwrap();
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

        game
    }
}
pub fn run_game(ren_path: &PathBuf, out: &PathBuf, game: &Game) -> Result<()> {
    println!("Running Game");
    if cfg!(target_os = "windows") {
    } else {
        Command::new(ren_path.to_str().unwrap())
            .args([out.join(game.name.clone()).to_str().unwrap(), "run"])
            .output()?;
    }
    Ok(())
}

use std::path::PathBuf;

use vina_story::{
    api::*,
    content::{Character, Game, Scene},
    generate_character_prompt, generate_story,
    music::{choose_music, fetch_music},
};

fn main() {
    /*
    let token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");

    let game = generate_story(&token).unwrap();
    println!("{:?}", game);

    for character in game.characters {
        let character_description = generate_character_prompt(&token, &character).unwrap();
        println!("{character_description}");
    }
    */

    let music_id = choose_music(vina_story::music::Theme::Romantic);
    fetch_music(&music_id, &PathBuf::from("./audio.mp3")).unwrap();
}

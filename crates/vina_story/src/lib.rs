//! Generate story from OpenAI

pub mod api;
pub mod content;
pub mod music;

use content::Location;

use crate::{
    api::*,
    content::{Character, Game, Scene},
};

pub fn generate_story(token: &str, prompt: &str) -> anyhow::Result<Game> {
    // Client to generate details of the story
    let mut story_client = ApiClient::new(token);

    story_client.run_prompt(prompt, None).unwrap();

    story_client
        .run_prompt("Generate a title for this story", None)
        .unwrap();

    let res = story_client.run_prompt("Limit the number of characters to a maximum of 3. Give me each of the characters in the story, along with detailed personality, clothing, and physical appearance details (include age, race, gender).", Some(get_characters_fn())).unwrap();

    let characters: Vec<Character> = parse_fncall(&res).unwrap();
    // println!("CHARACTERS {:?}", characters);

    let res = story_client.run_prompt("Limit the number of locations to a maximum of 5. Separate the story into multiple scenes, and for each scene give me a long and detailed description of the setting of the scene, omit any descriptions of people, include the name of the location, physical location it takes place in, objects and landmarks in the scene, mood, and time of day. Also create a title each scene that corresponds to the contents of the scene. Furthermore, for each scene, write me a script and return the result in a list with each element as a character's dialogue, and use a facial expression from this list: smiling, crying, nervous, excited, blushing to match the dialogue spoken. Also For each scene, tell me the music genre from this list Funky, Calm, Dark, Inspirational, Bright, Dramatic, Happy, Romantic, Angry, Sad", Some(get_scenes_fn())).unwrap();

    let scenes: Vec<Scene> = parse_fncall(&res).unwrap();
    // println!("SCENES {:?}", scenes);

    let game = Game {
        name: String::from("VinaGame"),
        synopsis: String::new(),
        characters,
        scenes,
    };
    Ok(game)
}

pub fn generate_character_prompt(token: &str, character: &Character) -> anyhow::Result<String> {
    generate_prompt(
        token,
        &format!("{}. {}", character.appearance, character.clothing),
    )
}

pub fn generate_location_prompt(token: &str, location: &Location) -> anyhow::Result<String> {
    generate_prompt(
        token,
        &format!(
            "{}. {}. {}. {}",
            location.description, location.landmarks, location.mood, location.time_of_day
        ),
    )
}

/// Pick youtube links given a description of the mood
pub fn generate_music_prompts(token: &str, prompt: Vec<&str>) -> anyhow::Result<Vec<String>> {
    let music_urls = vec![];

    let mut client = ApiClient::new(token);

    println!("Generating music...");

    Ok(music_urls)
}

fn generate_prompt(token: &str, prompt: &str) -> anyhow::Result<String> {
    let mut client = ApiClient::new(token);

    let role = r#"
        Today you are going to be an AI Artist. By that, I mean you gonna need to follow a ART PROMPT structure to make art. You are going to take my art requests.

        Here are a few prompt examples.

        PROMPT 1:

        1girl, Emma Watson as a bishoujo, big eyes, centered headshot portrait casual, indoors, standing,  bokeh,  Makoto Shinkai style

        PROMPT 2:

        best quality, ultra detailed, 1girl, Lucy Liu as a bishoujo, solo, standing, red hair, long braided hair, golden eyes, bangs, medium breasts, white shirt, necktie, stare, smile, looking at viewer, dark background

        PROMPT 3:

        1girl, Rihanna as a bishoujo, solo, masterpiece, high quality, professional full body photo, attractive woman, as hardcore hippy, toned physique, scifi, high quality, detailed eyes, eyelashes, Shaved Rainbow hair, slender waist, slender thighs, medium build, toned muscles, perfect face, ideal skin, photorealistic, beautiful clouds, night, cloudy , realistic, sharp focus, 8k high definition, insanely detailed, intricate, elegant
        
        Furthermore, the art you generate will be used as characters in a visual novel. 
        If the art request asks for a background, include the scenery/subject and then details, and finally the art style (realistic anime) inside the prompt.
        If the art request asks for a character, include the words "1person, solo, half body portrait, Makoto Shinkai style", and that the character looks similar to a random celebrity of your choice, the character's age, gender, clothing, appearance, additional details.

    "#;
    client.with_role(role);

    let res = client.run_prompt(prompt, None).unwrap();
    // let res = sd_prompt_client.run_prompt("Lily is a 25-year-old woman with fair, porcelain skin and sparkling emerald-green eyes that dance with mischief. Her wavy chestnut hair falls in cascades around her shoulders, framing her face with soft curls. Lily loves to dress in vibrant and eclectic clothing that reflects her creative spirit. She often adorns herself with colorful dresses, adorned with delicate lace and intricate patterns. She accessorizes with whimsical hats and mismatched socks, adding a touch of whimsy to her appearance.", None).unwrap();
    let content = parse_content(res).unwrap();

    // sd_prompt_client.run_prompt("Expo hall of a prestigious gaming conference in Tokyo. Booths showcasing various games, a stage for presentations, a food court. Excitement and anticipation. Afternoon", None).unwrap();
    Ok(content)
}

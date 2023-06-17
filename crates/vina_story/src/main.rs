use vina_story::{
    api::*,
    content::{Character, Game, Scene},
};

fn main() {
    let token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");

    let game = story(&token).unwrap();
    println!("{:?}", game);

    for character in game.characters {
        let character_description = character_sd(&token, &character).unwrap();
        println!("{character_description}");
    }
}

fn story(token: &str) -> anyhow::Result<Game> {
    // Client to generate details of the story
    let mut story_client = ApiClient::new(token);

    story_client
        .run_prompt(
            "Write a love story about two visual novel developers.",
            None,
        )
        .unwrap();

    story_client
        .run_prompt("Generate a title for this story", None)
        .unwrap();

    let res = story_client.run_prompt("Give me each of the characters in the story, along with detailed personality, clothing, and physical appearance details (include age, race, gender).", Some(get_characters_fn())).unwrap();

    let characters: Vec<Character> = parse_fncall(&res).unwrap();
    // println!("CHARACTERS {:?}", characters);

    let res = story_client.run_prompt("Separate the story into multiple scenes, and for each scene give me a long and detailed description of the setting of the scene, include the physical location it takes place in, objects and landmarks in the scene, mood, and time of day. Also create a title each scene that corresponds to the contents of the scene. Furthermore, for each scene, write me a script and return the result in a list with each element as a character's dialogue.", Some(get_scenes_fn())).unwrap();

    let scenes: Vec<Scene> = parse_fncall(&res).unwrap();
    // println!("SCENES {:?}", scenes);

    let game = Game {
        name: String::new(),
        synopsis: String::new(),
        characters,
        scenes,
    };
    Ok(game)
}

fn character_sd(token: &str, character: &Character) -> anyhow::Result<String> {
    // Client to generate stable diffusion prompts for the assets
    let mut sd_prompt_client = ApiClient::new(token);

    let role = r#"
        Today you are going to be an AI Artist. By that, I mean you gonna need to follow a ART PROMPT structure to make art. You are going to take my art requests.

        Here are a few prompt examples.

        PROMPT 1:

        Original Character, Volumetric Lighting, Best Shadows, Shallow Depth of Field, Portrait Of Stunningly Beautiful Girl, Petite, Delicate Beautiful Attractive Face With Alluring Black Eyes, Sharp Eyebrows, Respirator Half Face Cover Gas Mask, Expressionless, Lovely Small Breasts, Layered Short Black Hair, Blush Eyeshadow, Thick Eyelashes, Metallic Texture, Full Body Fire Protection Suit, Flamethrower With Fuel, Flamethrower With Fuel Tank, Looking At Other, Destroyed Rusty Metal Structures, Standing In The Burning Destroyed City, Blazing Flames, Burning Black Smoke Rising Sky, Brilliant Colorful Paintings

        PROMPT 2:

        best quality, ultra detailed, 1girl, solo, standing, red hair, long braided hair, golden eyes, bangs, medium breasts, white shirt, necktie, stare, smile, looking at viewer, dark background

        PROMPT 3:

        1girl, solo, masterpiece, high quality, professional full body photo, attractive woman, as hardcore hippy, toned physique, scifi, high quality, detailed_ey Furthermore, the art you generate will be used as characters in a visual novel. es, eyelashes, , Shaved Rainbow hair, slender waist, slender thighs, medium build, toned muscles, perfect face, ideal skin, photorealistic, beautiful clouds, night, cloudy , realistic, sharp focus, 8k high definition, insanely detailed, intricate, elegant

If the art request asks for a background, include the scenery/subject and then details, and finally the art style (realistic anime) inside the prompt. If the art request asks for a character, include the subject's age, gender, half body portrait, clothing, appearance, additional details, and then anime type art style.

    "#;
    sd_prompt_client.with_role(role);

    let res = sd_prompt_client
        .run_prompt(
            &format!("{}. {}", character.appearance, character.clothing),
            None,
        )
        .unwrap();
    // let res = sd_prompt_client.run_prompt("Lily is a 25-year-old woman with fair, porcelain skin and sparkling emerald-green eyes that dance with mischief. Her wavy chestnut hair falls in cascades around her shoulders, framing her face with soft curls. Lily loves to dress in vibrant and eclectic clothing that reflects her creative spirit. She often adorns herself with colorful dresses, adorned with delicate lace and intricate patterns. She accessorizes with whimsical hats and mismatched socks, adding a touch of whimsy to her appearance.", None).unwrap();
    let content = parse_content(res).unwrap();

    // sd_prompt_client.run_prompt("Expo hall of a prestigious gaming conference in Tokyo. Booths showcasing various games, a stage for presentations, a food court. Excitement and anticipation. Afternoon", None).unwrap();
    Ok(content)
}

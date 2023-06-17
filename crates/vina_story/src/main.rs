use vina_story::{api::*, content::Character};

fn main() {
    let token = std::env::var("OPENAI_KEY").expect("Could not get OPENAI_KEY");
    let mut client = ApiClient::new(token);

    client
        .run_prompt(
            "Write a love story about two visual novel developers.",
            None,
        )
        .unwrap();

    client
        .run_prompt("Generate a title for this story", None)
        .unwrap();

    let res = client.run_prompt("Give me each of the characters in the story, along with detailed personality, clothing, and physical appearance details (include age, race, gender).", Some(get_characters_fn())).unwrap();

    let characters: Vec<Character> = parse_message(&res).unwrap();
    println!("CHARACTERS {:?}", characters);

    let res = client.run_prompt("Separate the story into multiple scenes, and for each scene give me a detailed description of the physical location it takes place in. Also create a title that corresponds to the contents of the scene. Furthermore, for each scene, write me a script and return the result in a list with each element as a character's dialogue.", Some(get_scenes_fn())).unwrap();
}

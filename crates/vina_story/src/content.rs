//! Game content

/// Configurable aspects when generating the dialogue
pub struct Settings {
    // dialogue length (how long and detaile deach scene should be)
    // art style
}

pub struct Game {
    /// Title of the game
    pub name: String,
    /// Brief summary for the game
    pub synopsis: String,
    /// Characters that are present in the story
    pub characters: Vec<Character>,
}

pub struct Character {
    pub name: String,
    /// Description of the character's personality. Will be taken into account when generating
    /// dialogue
    pub personality: String,
    /// Description of the character's clothing choices.
    pub clothing: String,
    /// Description of the character's physical appearence.
    pub appearence: String,
}

/// Description of major plot points
pub struct Plot {
    pub scenes: Vec<Scene>,
}

pub struct Scene {
    /// Name of the scene
    pub title: String,
    /// Description of the physical location the scene takes place in
    pub setting: String,
    pub script: Script,
    /// Description of the modd of the scene, taken into account when generating the background
    /// location as well as music
    pub mood: String,
}

pub struct Script {
    /// Lines of dialogue that are present in the story
    pub dialogues: Vec<Dialogue>,
}

pub struct Dialogue {
    /// Name of character that is speaking
    pub character: Option<String>,
    /// Actual text content of the dialogue line
    pub dialogue: String,
    /// Description of which character sprite to use
    pub mood: String,
}

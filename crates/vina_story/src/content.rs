//! Game content

use serde::{de::DeserializeOwned, Deserialize};

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

#[derive(Deserialize, Debug)]
pub struct Character {
    pub name: String,
    /// Description of the character's personality. Will be taken into account when generating
    /// dialogue
    pub personality: String,
    /// Description of the character's clothing choices.
    pub clothing: String,
    /// Description of the character's physical appearence.
    pub appearance: String,
}

/// Description of major plot points
pub struct Plot {
    pub scenes: Vec<Scene>,
}

#[derive(Deserialize, Debug)]
pub struct Scene {
    /// Name of the scene
    pub title: String,
    /// Description of the physical location the scene takes place in
    pub location: Location,
    pub script: Vec<Dialogue>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    /// Physical description of the location the scene takes place in
    pub description: String,
    /// Concrete objects and landmarks in the scene
    pub landmarks: String,
    /// Information on the mood and time of day
    pub mood: String,
    /// Time of day
    pub time_of_day: String,
}

#[derive(Deserialize, Debug)]
pub struct Dialogue {
    /// Name of character that is speaking
    pub speaker: String,
    /// Actual text content of the dialogue line
    pub content: String,
    // /// Description of which character sprite to use
    // pub mood: String,
}

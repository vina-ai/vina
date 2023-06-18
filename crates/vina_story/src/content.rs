//! Game content

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::music::Theme;

/// Configurable aspects when generating the dialogue
pub struct Settings {
    // dialogue length (how long and detaile deach scene should be)
    // art style
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    /// Title of the game
    pub name: String,
    /// Brief summary for the game
    pub synopsis: String,
    /// Characters that are present in the story
    pub characters: Vec<Character>,
    /// Scenes in the game
    pub scenes: Vec<Scene>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    /// Name of the scene
    pub title: Option<String>,
    /// Description of the physical location the scene takes place in
    pub location: Location,
    pub script: Vec<Dialogue>,
    /// Genre of music to be played in the scene
    pub music: Theme,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    /// Name of the location
    pub name: String,
    /// Physical description of the location the scene takes place in
    pub description: String,
    /// Concrete objects and landmarks in the scene
    pub landmarks: String,
    /// Time of day
    pub time_of_day: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dialogue {
    /// Name of character that is speaking
    pub speaker: String,
    /// Facial expression of the character speaking
    pub facial_expression: String,
    /// Actual text content of the dialogue line
    pub content: String,
    // /// Description of which character sprite to use
    // pub mood: String,
}

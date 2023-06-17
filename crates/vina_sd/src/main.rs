use std::{
    fs::{self, File},
    io::{BufWriter, Cursor, Read, Seek, Write},
    path::PathBuf,
};

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use image::{io::Reader as ImageReader, ImageOutputFormat};
use serde_json::{json, Value};
use vina_sd::api::{write_to_file, ApiClient, NEGATIVE_PROMPT};
use vina_story::content::Character;

fn main() {
    let api_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");
    let client = ApiClient::new(api_url);

    // generate_character(&client);
}

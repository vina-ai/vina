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

fn main() {
    let api_url = std::env::var("NOVELAI_URL").expect("Could not get NOVELAI_URL");
    let client = ApiClient::new(api_url);

    println!("Generating base character...");
    let body: Value = json!({
        "prompt": "1girl, bishoujo, casual, indoors, standing, 25 years old, fair skin, emerald-green eyes, wavy chestnut hair, cascading curls, vibrant and eclectic clothing, colorful dresses with delicate lace and intricate patterns, whimsical hats, mismatched socks, creative spirit, full body portrait, no background, anime art style.",
        "negative_prompt": NEGATIVE_PROMPT,
        "seed": -1,
        "steps": 28,
        "width": 512,
        "height": 512,
        "sampler_index": "Euler",
        "cfg_scale": 12
    });
    let base = client.request(body, "sdapi/v1/txt2img").unwrap();

    let path = PathBuf::from("./output.png");
    // write_to_file(base, &path).unwrap();

    //     let img = ImageReader::open("output.png").unwrap().decode().unwrap();
    //     let mut image_data: Vec<u8> = Vec::new();
    //     img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
    //         .unwrap();
    //     let res_base64 = base64::encode(image_data);

    println!("Stripping background...");
    let body: Value = json!({
        "init_images": vec![format!("data:image/png;base64,{base}")],
        "prompt": "[txt2mask mode='add' method='clipseg']background[/txt2mask]",
        "negative_prompt": NEGATIVE_PROMPT,
        "seed": -1,
        "steps": 20,
        "width": 512,
        "height": 512,
        "sampler_index": "Euler a",
        "cfg_scale": 12,

        "script_name": "ABG Remover",
        "script_args": [
            true, // Only save background free pictures
            false, // Do not auto save
            false, // Custom Background
            Value::Null, // Background Color
            false // Random Custom Background
        ],
    });
    let base_rembg = client.request(body, "sdapi/v1/img2img").unwrap();
    write_to_file(base_rembg, &PathBuf::from("./output-rembg.png")).unwrap();
}

//! Wrapper around NovelAI's api
//!x
//!

use std::{time::Duration, str::Split};

use reqwest::{blocking, header::AUTHORIZATION};
use serde_json::{json, Value};
use image_base64::from_base64;
use image::{ImageFormat, DynamicImage};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

pub struct ApiClient {
    api_url: String,
    client: reqwest::blocking::Client,
}

impl ApiClient {
    pub fn new(api_url: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();
        Self {
            api_url,
            client,
        }
    }

    fn request(&self) -> anyhow::Result<()> {
        let body: Value = json!({
            "hr_negative_prompt": "nsfw, distorted eyes, cropped hair, off center, left, right, ugly, distorted, scary, watermarks, caption, crop, aspect ratio distortion, blurry, harsh lines, black lines, smudged, duplicate, morbid, mutilated, out of frame, off center, extra fingers, mutilated hands, poorly drawn hands, mutation, deformed, bad anatomy, gross proportions, malformed limbs, missing arms, missing legs, extra arms, extra legs, mutated hands, fused fingers, too many fingers, long neck, 2 heads, 2 faces, V deformed arm, toy, different eyes, extra ears, double copying of elements face, grain, duplicate, multi, copy",
            "seed": -1,
            "prompt": "1girl, bishoujo, casual, indoors, standing, 25 years old, fair skin, emerald-green eyes, wavy chestnut hair, cascading curls, vibrant and eclectic clothing, colorful dresses with delicate lace and intricate patterns, whimsical hats, mismatched socks, creative spirit, full body portrait, no background, anime art style.",
            "steps": 28,
            "width": 512,
            "height": 512,
            "sampler_index": "Euler",
            "cfg_scale": 12
        });

        let mut res: Value = self
            .client
            .post(self.api_url.clone())
            .json(&body)
            .send()?
            .json()?;

        // Extract the response images
        let images = res["images"]
            .as_array_mut()
            .expect("Expected image array");

        for i in images.iter_mut() {
            let split_i: Vec<&str> = i.as_str().unwrap().split(",").collect();
            let base64 = split_i.get(0).unwrap().to_string();
            let bytes = general_purpose::STANDARD.decode(base64).unwrap();
            match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
                Ok(_dynamic_image) => {
                    println!("PNG image is successfully decoded");
                    std::fs::write("output.png", bytes).unwrap();
                }
                Err(_) => {
                    println!("Incorrect format of the image");
                }
            }
        }

        Ok(())
    }

    pub fn test(&self) -> anyhow::Result<()> {
        self.request().unwrap();
        Ok(())
    }

}

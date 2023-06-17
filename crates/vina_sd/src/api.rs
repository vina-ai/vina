//! Wrapper around NovelAI's api
//!x
//!

use std::{path::Path, str::Split, time::Duration};

use anyhow::anyhow;
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use image::{DynamicImage, ImageFormat};
use reqwest::{blocking, header::AUTHORIZATION};
use serde_json::{json, Value};

pub struct ApiClient {
    api_url: String,
    client: reqwest::blocking::Client,
}

pub const NEGATIVE_PROMPT: &'static str =  "nsfw, distorted eyes, cropped hair, off center, left, right, ugly, distorted, scary, watermarks, caption, crop, aspect ratio distortion, blurry, harsh lines, black lines, smudged, duplicate, morbid, mutilated, out of frame, off center, extra fingers, mutilated hands, poorly drawn hands, mutation, deformed, bad anatomy, gross proportions, malformed limbs, missing arms, missing legs, extra arms, extra legs, mutated hands, fused fingers, too many fingers, long neck, 2 heads, 2 faces, V deformed arm, toy, different eyes, extra ears, double copying of elements face, grain, duplicate, multi, copy";

impl ApiClient {
    pub fn new(api_url: String) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();
        Self { api_url, client }
    }

    pub fn request(&self, prompt: &str, output: &Path) -> anyhow::Result<()> {
        let body: Value = json!({
            "prompt": prompt,
            "negative_prompt": NEGATIVE_PROMPT,
            "seed": -1,
            "steps": 28,
            "width": 512,
            "height": 512,
            "sampler_index": "Euler",
            "cfg_scale": 12
        });

        let response = self
            .client
            .post(format!("{}/sdapi/v1/txt2img", self.api_url))
            .json(&body)
            .send()?;

        if !response.status().is_success() {
            let err_msg = response.text()?;
            return Err(anyhow!("{}", err_msg));
        }

        let mut res: Value = response.json()?;

        // Extract the response images
        let images = res["images"].as_array_mut().expect("Expected image array");

        for i in images.iter_mut() {
            let split_i: Vec<&str> = i.as_str().unwrap().split(",").collect();
            let base64 = split_i.get(0).unwrap().to_string();
            let bytes = general_purpose::STANDARD.decode(base64).unwrap();
            match image::load_from_memory_with_format(&bytes, ImageFormat::Png) {
                Ok(_dynamic_image) => {
                    std::fs::write(output, bytes).unwrap();
                },
                Err(_) => {
                    println!("Incorrect format of the image");
                },
            }
        }

        Ok(())
    }
}

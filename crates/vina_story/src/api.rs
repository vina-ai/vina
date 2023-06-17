//! Wrapper around OpenAI's api
//!
//!

use std::collections::HashMap;

use reqwest::{blocking, header::AUTHORIZATION};
use serde_json::{json, Map, Value};

pub const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

pub struct ApiClient {
    api_key: String,
    model: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: String::from("gpt-3"),
        }
    }

    pub fn test(&self) -> anyhow::Result<()> {
        let client = reqwest::blocking::Client::new();

        let body: serde_json::Value = json!({
         "model": "gpt-3.5-turbo",
         "messages": [{"role": "user", "content": "Say this is a test!"}],
         "temperature": 0.7
        });

        let res = client
            .post(API_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.clone()))
            .json(&body)
            .send()?
            .text()?;

        println!("{:?}", res);
        Ok(())
    }
}

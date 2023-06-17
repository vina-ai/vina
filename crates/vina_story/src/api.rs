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
            model: String::from("gpt-3.5-turbo-0613"),
        }
    }

    pub fn test(&self) -> anyhow::Result<()> {
        let client = reqwest::blocking::Client::new();

        let body: serde_json::Value = json!({
            "model": self.model,
            "messages": [
                {"role": "user", "content": "What's the weather in Kitchener Waterloo Region Canada?"}
            ],
            "functions": [
                {
                    "name": "get_current_weather",
                    "description": "Get the current weather in a given location",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "location": {
                                "type": "string",
                                "description": "The city and state, e.g. San Francisco, CA",
                            },
                            "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]},
                        },
                        "required": ["location"],
                    }
                }
            ],
        });

        let res = client
            .post(API_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.clone()))
            .json(&body)
            .send()?
            .text()?;

        let res_val: Value = serde_json::from_str(&res)?;
        println!("{}", serde_json::to_string_pretty(&res_val)?);

        let choices = &res_val["choices"].as_array().unwrap();
        for choice in choices.iter() {
            let finish_reason = choice["finish_reason"].as_str().unwrap();
            if finish_reason == "function_call" {
                let fn_call = &choice["message"]["function_call"];
                let fn_name = fn_call["name"].as_str().unwrap();
                let fn_args = &fn_call["arguments"];
                if fn_name == "get_current_weather" {
                    println!("Calling {} with {:?}", fn_name, fn_args);
                }
            }
        }
        Ok(())
    }

    pub fn parse_response(&self, res: &Value) -> anyhow::Result<()> {
        Ok(())
    }
}

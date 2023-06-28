//! Client for openai's chat api
//!
//! Use [schemars](https://docs.rs/schemars/latest/schemars/index.html#macros)

use std::{collections::HashMap, time::Duration};

use lazy_static::lazy_static;
use reqwest::{blocking, header::AUTHORIZATION};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Map, Value};

pub struct ApiClient {
    api_key: String,
    model: String,
    client: reqwest::blocking::Client,
    /// Message history
    messages: Vec<Value>,
}

impl ApiClient {
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();
        Self {
            api_key: api_key.into(),
            model: String::from("gpt-3.5-turbo-0613"),
            client,
            messages: Vec::new(),
        }
    }

    /// Run the api request to openai
    fn request(&self, function: Option<Value>) -> anyhow::Result<Value> {
        // TODO this is current a pretty dumb solution (api doesnt work when passing in functions
        // as empty vec)
        let body: Value = if let Some(function) = function {
            json!({
                "model": self.model,
                "messages": self.messages,
                "functions": vec![function]
            })
        } else {
            json!({
                "model": self.model,
                "messages": self.messages,
            })
        };

        let mut res: Value = self
            .client
            .post(API_URL)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.clone()))
            .json(&body)
            .send()?
            .json()?;

        println!("{}", serde_json::to_string_pretty(&res)?);

        // TODO check if response is error

        // Extract the response message to add to chat context
        let choices = res["choices"]
            .as_array_mut()
            .expect("Expected choice array");

        assert!(choices.len() == 1);

        // TODO currently not sure why choices is an array
        let choice = choices.get_mut(0).expect("Expected to get first choice");
        let res_msg = choice["message"].take();

        Ok(res_msg)
    }

    /// Send one prompt to openai, can optionally include a function to be used to extract data
    pub fn run_prompt(&mut self, prompt: &str, function: Option<Value>) -> anyhow::Result<&Value> {
        self.messages.push(json!({
            "role": "user", "content": prompt,
        }));

        let res_msg = self.request(function).unwrap();

        self.messages.push(res_msg);

        Ok(self.messages.last().unwrap())
    }

    /// Specify a description of the role you wish openapi to take on
    pub fn with_role(&mut self, role: &str) {
        self.messages.push(json!({
            "role": "system", "content": role,
        }));
    }
}

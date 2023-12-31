//! Wrapper around OpenAI's api
//!
//!

use std::{collections::HashMap, time::Duration};

use lazy_static::lazy_static;
use reqwest::{blocking, header::AUTHORIZATION};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Map, Value};

use crate::content::Character;

pub const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

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

/// Parse a function call
pub fn parse_fncall<T: DeserializeOwned>(msg: &Value) -> anyhow::Result<T> {
    let fn_args = parse_fncall_raw(msg)?;
    let downcasted = serde_json::from_value(fn_args)?;
    Ok(downcasted)
}

pub fn parse_fncall_raw(msg: &Value) -> anyhow::Result<Value> {
    let fn_call = &msg["function_call"];
    let fn_name = fn_call["name"].as_str().unwrap();

    // TODO hardcoded inner key (since most of time we only have one argument)
    let fn_args = fn_call["arguments"].as_str().unwrap();
    let mut fn_args: Value = serde_json::from_str(fn_args).unwrap();
    let fn_args = fn_args["inner"].take();
    Ok(fn_args)
}

/// Parse text content
pub fn parse_content(msg: &Value) -> anyhow::Result<String> {
    let content = msg["content"].as_str().unwrap().to_string();
    Ok(content)
}

pub fn get_characters_fn() -> Value {
    json!({
        "name": "get_characters",
        "description": "Get detailed information about each character in the story",
        "parameters": {
            "type": "object",
            "properties": {
                "inner": {
                    "type": "array",
                    "description": "List of characters in the story",
                    "items": {
                        "type": "object",
                        "description": "Detailed information about a character",
                        "properties": {
                            "name": {
                                "type": "string",
                                "description:": "Name of the character",
                            },
                            "personality": {
                                "type": "string",
                                "description:": "Explanation of the character's personality traits",
                            },
                            "clothing": {
                                "type": "string",
                                "description:": "Explanation of the character's clothing choices and what they wear",
                            },
                            "appearance": {
                                "type": "string",
                                "description:": "Explanation of the character's physical appearance",
                            },
                        }
                    },
                },
            },
            "required": ["inner"],
        }
    })
}

pub fn get_scenes_fn() -> Value {
    json!({
        "name": "get_scenes",
        "description": "Extract detailed information about scenes in the story",
        "parameters": {
            "type": "object",
            "properties": {
                "inner": {
                    "type": "array",
                    "description": "List of scenes in the story",
                    "items": {
                        "type": "object",
                        "properties": {
                            "title": {
                                "type": "string",
                                "description:": "Descriptive title of the scene based on it's contents",
                            },
                            "music": {
                                "enum": [
                                    "Funky", "Calm", "Dark", "Inspirational", "Bright", "Dramatic", "Happy", "Romantic", "Angry", "Sad"
                                ],
                                "description": "Genre of music that should be played in this scene",
                            },
                            "location": {
                                "type": "object",
                                "description:": "Description of the physical location the scene takes place in. Omit any descriptions of people.",
                                "properties": {
                                    "name": {
                                        "type": "string",
                                        "description": "Name of the location",
                                    },
                                    "description": {
                                        "type": "string",
                                        "description": "Description of the physical location and objects in the scene. Omit any descriptions of people.",
                                    },
                                    "landmarks": {
                                        "type": "string",
                                        "description": "Landmarks and objects of focus that are present in the scene. Omit any descriptions of people.",
                                    },
                                    "time_of_day": {
                                        "type": "string",
                                        "description": "What time of day it is",
                                    },
                                }
                            },
                        }
                    }
                },
            },
            "required": ["inner"],
        }
    })
}

pub fn get_script_fn() -> Value {
    json!({
        "name": "get_script_fn",
        "description": "Script to be used in scene",
        "parameters": {
            "type": "object",
            "properties": {
                "inner": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "description": "A line in the script, contains information like the speaker, what is being said, and facial expression",
                        "properties": {
                            "speaker": {
                                "type": "string",
                                "description": "Name of the speaker"
                            },
                            "facial_expression": {
                                "type": "string",
                                "description": "Use an emotion from this list: smiling, crying, nervous, excited, blushing to match the dialogue spoken"
                            },
                            "content": {
                                "type": "string",
                                "description": "What the speaker says"
                            }
                        }
                    }
                },

            },
            "required": ["inner"],
        }
    })
}

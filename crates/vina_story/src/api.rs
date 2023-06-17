//! Wrapper around OpenAI's api
//!
//!

use std::{collections::HashMap, time::Duration};

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
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        let prompt = r#"
            Write a love story about two visual novel developers. Give me each of the characters in the story, along with detailed personality, clothing, and appearance details. Next separate the story into multiple scenes, and for each scene give me a detailed description of the physical location it takes place in. Also create a title that corresponds to the contents of the scene. Furthermore, for each scene, write me a script and return the result in a list with each element as a character's dialogue.
        "#;

        let body: serde_json::Value = json!({
            "model": self.model,
            "messages": [
                {"role": "user", "content": prompt},
            ],
            "functions": [
                {
                    "name": "get_story",
                    "description": "Extract detailed information about characters, settings, and script from the story",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "characters": {
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
                                }
                            },

                            "scenes": {
                                "type": "object",
                                "description": "List of scenes in the story",
                                "properties": {
                                    "title": {
                                        "type": "string",
                                        "description:": "Descriptive title of the scene based on it's contents",
                                    },
                                    "setting": {
                                        "type": "string",
                                        "description:": "Description of the physical location the scene takes place in",
                                    },
                                    "script": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "description": "A line in the script, contains information like the speaker and also what is being said",
                                            "properties": {
                                                "speaker": {
                                                    "type": "string",
                                                    "description": "Name of the speaker"
                                                },
                                                "content": {
                                                    "type": "string",
                                                    "description": "What the speaker actually says"
                                                }
                                            }
                                        }

                                    },
                                }
                            },

                        },
                        "required": ["characters", "scenes"],
                    }
                },
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

        /*
        let choices = &res_val["choices"].as_array().unwrap();
        for choice in choices.iter() {
            let finish_reason = choice["finish_reason"].as_str().unwrap();
            if finish_reason == "function_call" {
                let fn_call = &choice["message"]["function_call"];
                let fn_name = fn_call["name"].as_str().unwrap();
                let fn_args = fn_call["arguments"].as_str().unwrap();
                let fn_args: Value = serde_json::from_str(fn_args).unwrap();
                if fn_name == "get_current_weather" {
                    println!("Calling {} with {:?}", fn_name, fn_args);
                    get_current_weather(&fn_args);
                }
            }
        }
        */
        Ok(())
    }

    pub fn parse_response(&self, res: &Value) -> anyhow::Result<()> {
        Ok(())
    }
}

fn get_current_weather(param: &Value) {
    let location = param["location"].as_str().unwrap();
    println!("getting the current weather in {location}...");
}

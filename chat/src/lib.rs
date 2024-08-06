use cliclack::{Confirm, Input, Password};
use encrypt_config::{Config, SecretSource};
use futures_util::StreamExt;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::io::{self, Write as _};
use std::process::Command;
use url::Url;

const SYSTEM: &str = r#"I need you solve the following latex formula with python's **sympy** library.
You should only give me a python function with signature `formula()` which returns the simplified formula.
And I only need the python function code, please do **not** answer or do anything else like interpreting code or call `solution`."#;

#[derive(Default)]
pub struct Solver {
    client: Client,
    config: Config,
}

#[derive(Serialize, Deserialize, SecretSource)]
#[source(name = "sym_gpt_cfg", keyring_entry = "sym_gpt")]
struct Secret {
    endpoint: Url,
    api_key: String,
}

impl Default for Secret {
    fn default() -> Self {
        Self {
            endpoint: Url::parse("https://api.openai.com/v1/chat/completions").unwrap(),
            api_key: "".to_owned(),
        }
    }
}

impl Solver {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            config: Config::default(),
        }
    }

    fn req() -> Req {
        Req {
            stream: true,
            model: ModelVersion::GPT4o,
            messages: vec![
                Message {
                    role: Role::System,
                    content: SYSTEM.to_owned(),
                },
                Message {
                    role: Role::User,
                    content: Input::new("Latex Formula")
                        .multiline()
                        .required(true)
                        .interact()
                        .expect("Failed to get input"),
                },
            ],
        }
    }

    pub async fn run(&self) {
        let secret = self.config.get::<Secret>();
        let mut stdout = io::stdout();

        loop {
            let mut python_code = String::new();
            let mut stream = self
                .client
                .post(secret.endpoint.clone())
                .header(header::AUTHORIZATION, format!("Bearer {}", secret.api_key))
                .json(&Self::req())
                .send()
                .await
                .unwrap()
                .bytes_stream();
            while let Some(item) = stream.next().await {
                let item = item.unwrap();
                let chunk = std::str::from_utf8(&item).expect("Invalid UTF-8 sequence");
                for chunk in chunk.split("\n\n") {
                    if let Some(chunk) = chunk.strip_prefix("data: ") {
                        if chunk == "[DONE]" {
                            break;
                        }
                        if let Some(chunk) = serde_json::from_str::<Chunk>(chunk).unwrap().content()
                        {
                            python_code.push_str(&chunk);
                            stdout.write_all(chunk.as_bytes()).unwrap();
                            stdout.flush().unwrap();
                        }
                    }
                }
            }
            stdout.write_all(b"\n").unwrap();

            if !Confirm::new("Re-generate?").interact().unwrap() {
                let sym_py_path = std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("dist/sym.py");

                python_code = python_code.lines().fold(String::new(), |acc, line| {
                    if !line.contains("```") {
                        format!("{}\n{}", acc, line)
                    } else {
                        acc
                    }
                });
                cliclack::outro("Result:").unwrap();
                Command::new("python")
                    .args([
                        sym_py_path.as_os_str().to_str().unwrap(),
                        python_code.trim(),
                    ])
                    .status()
                    .unwrap();
            }
        }
    }

    pub fn set_config(&mut self) {
        let mut secret = self.config.get_mut::<Secret>();
        secret.endpoint = Input::new("Endpoint")
            .default_input(secret.endpoint.as_str())
            .interact()
            .unwrap();
        secret.api_key = Password::new("API key").interact().unwrap();
    }
}

#[derive(Debug, Serialize)]
struct Req {
    stream: bool,
    model: ModelVersion,
    messages: Messages,
}

type Messages = Vec<Message>;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
enum ModelVersion {
    #[default]
    #[serde(rename = "gpt-4o")]
    GPT4o,
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Chunk {
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Choice {
    delta: Delta,
    finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
struct Delta {
    role: Option<Role>,
    content: Option<String>,
}

impl Chunk {
    fn content(&self) -> Option<String> {
        self.choices.first().and_then(|c| c.delta.content.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

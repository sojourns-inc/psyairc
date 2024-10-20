use irc::client::prelude::*;
use std::env;
use tokio;
use futures::StreamExt;
use reqwest;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize)]
struct PsyAIRequest {
    question: String,
    temperature: u8,
    tokens: u16,
    drug: bool,
    model: String,
    version: String,
}

#[derive(Deserialize)]
struct PsyAIResponse {
    assistant: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let server = "<SERVER>";
    let port = 6667;
    let nick = "psypup";
    let channel = "<CHANNEL>";

    let config = Config {
        nickname: Some(nick.to_owned()),
        username: Some(nick.to_owned()),
        realname: Some("PsyPup Harm Redux Bot".to_owned()),
        server: Some(server.to_owned()),
        port: Some(port),
        channels: vec![channel.to_owned()],
        use_tls: Some(false),
        ..Default::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    println!("Connected to IRC server and joined channel {}", channel);

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                if target == channel && msg.contains("psypup") {
                    let query = msg
                        .replace(&format!("{}:", nick), "")
                        .replace(nick, "")
                        .trim()
                        .to_string();

                    let sender_nick = message.source_nickname().unwrap_or("unknown");

                    if !query.is_empty() {
                        match fetch_question_from_psyai(&query).await {
                            Ok(assistant_response) => {
                                let _ = client
                                    .send_privmsg(
                                        target,
                                        format!("{}: {}", sender_nick, assistant_response),
                                    );
                            }
                            Err(_) => {
                                let _ = client
                                    .send_privmsg(
                                        target,
                                        format!(
                                            "{}: Sorry, I couldn't process that request.",
                                            sender_nick
                                        ),
                                    );
                            }
                        }
                    } else {
                        let _ = client
                            .send_privmsg(
                                target,
                                format!("{}: Please provide a query after tagging me.", sender_nick),
                            );
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}

async fn fetch_question_from_psyai(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = env::var("BASE_URL_BETA")?;

    let request_body = PsyAIRequest {
        question: format!("{}\n\nRespond in a single sentence.", query),
        temperature: 0,
        tokens: 100,
        drug: false,
        model: "openai-next".to_string(),
        version: "v2".to_string(),
    };

    let client = reqwest::Client::new();

    let res = client
        .post(&format!("{}/q", base_url))
        .json(&request_body)
        .send()
        .await?;

    if res.status().is_success() {
        let resp_json: PsyAIResponse = res.json().await?;
        Ok(resp_json.assistant)
    } else {
        Err("Failed to get a valid response from PsyAI".into())
    }
}

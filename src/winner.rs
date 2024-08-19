use fantoccini::ClientBuilder;
use poise::serenity_prelude::Result;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn process(url: String) -> Result<String, Error> {
    let data = fetch_html(&url).await?;
    let output = format!(
        "Result:\nDate of Duel: {}\nRPS winner: {}\nWinner: {}\nOutcome: {} {}-{} {}",
        data.date.trim(),
        data.rps_winner.trim(),
        data.winner.trim(),
        data.player1.trim(),
        data.wins1.trim(),
        data.wins2.trim(),
        data.player2.trim()
    );
    Ok(output)
}

#[derive(Serialize, Deserialize)]
struct Stats {
    date: String,
    rps_winner: String,
    winner: String,
    player1: String,
    player2: String,
    wins1: String,
    wins2: String,
}

async fn fetch_html(url: &str) -> Result<Stats, Error> {
    let content = content(url).await.unwrap();
    let json = content;
    let mut stats = Stats {
        date: String::new(),
        rps_winner: String::new(),
        winner: String::new(),
        player1: String::new(),
        player2: String::new(),
        wins1: String::new(),
        wins2: String::new(),
    };
    stats.date = json["date"].to_string();
    stats.player1 = json["player1"]["username"].to_string();
    stats.player2 = json["player2"]["username"].to_string();
    let mut wins1 = 0;
    let mut wins2 = 0;
    if let Some(plays) = json["plays"].as_array() {
        for play in plays {
            if play["play"].to_string() == "\"RPS\"" {
                stats.rps_winner = play["winner"].to_string();
            }
            if play["play"].to_string() == "\"Admit defeat\"" {
                if stats.player1 == play["username"].to_string() {
                    wins2 += 1;
                } else {
                    wins1 += 1
                }
                if play["over"].to_string() == "true" {
                    break;
                }
            }
        }
    }
    if stats.player1.starts_with('"') && stats.player1.ends_with('"') {
        stats.player1 = stats.player1[1..stats.player1.len() - 1].to_string();
    }
    if stats.player2.starts_with('"') && stats.player2.ends_with('"') {
        stats.player2 = stats.player2[1..stats.player2.len() - 1].to_string();
    }
    if stats.rps_winner.starts_with('"') && stats.rps_winner.ends_with('"') {
        stats.rps_winner = stats.rps_winner[1..stats.rps_winner.len() - 1].to_string();
    }
    if stats.date.starts_with('"') && stats.date.ends_with('"') {
        stats.date = stats.date[1..stats.date.len() - 1].to_string();
    }
    stats.wins1 = wins1.to_string();
    stats.wins2 = wins2.to_string();
    stats.winner = if wins1 > wins2 {
        stats.player1.clone()
    } else if wins1 < wins2 {
        stats.player2.clone()
    } else {
        "Draw".to_string()
    };
    Ok(stats)
}

async fn content(url: &str) -> Result<Value, fantoccini::error::CmdError> {
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("Failed to create client");
    client.new_window(false).await?;
    client
        .switch_to_window(
            client
                .windows()
                .await?
                .remove(client.windows().await?.len() - 1),
        )
        .await?;
    client.goto(url).await?;
    client
        .execute(
            r#"
        (function() {
            var originalOpen = XMLHttpRequest.prototype.open;
            var originalSend = XMLHttpRequest.prototype.send;
            XMLHttpRequest.prototype.open = function(method, url, async) {
                this._url = url; // Store the URL
                this._method = method; // Store the method
                originalOpen.apply(this, arguments);
            };
            XMLHttpRequest.prototype.send = function(body) {
                var xhr = this;
                this.addEventListener('load', function() {
                    if (xhr._url.includes('view-replay')) { // Check if it's the relevant request
                        window.replayResponse = xhr.responseText; // Store the response
                    }
                });
                originalSend.apply(this, arguments);
            };
        })();
        "#,
            vec![],
        )
        .await?;
    client.execute("grecaptcha.ready(function() {
            grecaptcha.execute('6LcjdkEgAAAAAKoEsPnPbSdjLkf4bLx68445txKj', {action: 'submit'}).then(function(token) {
                loadReplay(token);
            });
        });",
    vec![]).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    let response = client
        .execute("return window.replayResponse;", vec![])
        .await?;
    let response_string = response.to_string()[1..response.to_string().len() - 1]
        .replace("\\\\\\\"", "\\\\\"")
        .replace("\\\"", "\"");
    let response: Value = serde_json::from_str(&response_string).unwrap();
    client.close_window().await?;
    Ok(response)
}

use std::collections::HashMap;

use fantoccini::ClientBuilder;
use poise::serenity_prelude::Result;
use reqwest::Error;
use serde_json::Value;

pub async fn deck(url: String) -> Result<String, Error> {
    let data = fetch_html(&url).await?;
    let empty = String::from("");
    let mut deck1 = String::new();
    for id in data.deck1 {
        let card = data.ids.get(&id).unwrap_or(&empty);
        if card != &empty {
            deck1.push_str(&card);
            deck1.push_str("\n");
        }
    }
    let mut deck2 = String::new();
    for id in data.deck2 {
        let card = data.ids.get(&id).unwrap_or(&empty);
        if card != &empty {
            deck2.push_str(&card);
            deck2.push_str("\n");
        }
    }
    let output = format!("{}:\n{}\n{}:\n{}", data.player1, deck1, data.player2, deck2);
    Ok(output)
}

pub async fn process(url: String) -> Result<String, Error> {
    let data = fetch_html(&url).await?;
    let output = format!(
        "Date of Duel: {}\nLength: {}\nRPS winner: {}\nWinner: ***{}***\nOutcome: {} {}-{} {}\nDecks:\n\t{}:\n\t  {}, ID: **{}**\n\t{}:\n\t  {}, ID: **{}**\nLink: [DuelingBook](<{}>)",
        data.date,
        data.duration,
        data.rps_winner,
        data.winner,
        data.player1,
        data.wins1,
        data.wins2,
        data.player2,
        data.player1,
        data.decksize1,
        data.deckid1,
        data.player2,
        data.decksize2,
        data.deckid2,
        url
    );
    Ok(output)
}

struct Stats {
    date: String,
    rps_winner: String,
    winner: String,
    player1: String,
    player2: String,
    wins1: String,
    wins2: String,
    duration: String,
    decksize1: String,
    decksize2: String,
    deckid1: String,
    deckid2: String,
    ids: HashMap<u32, String>,
    deck1: Vec<u32>,
    deck2: Vec<u32>,
}

async fn fetch_html(url: &str) -> Result<Stats, Error> {
    let (content, fingerprint) = content(url).await.unwrap();
    let json = content;
    let mut stats = Stats {
        date: String::new(),
        rps_winner: String::new(),
        winner: String::new(),
        player1: String::new(),
        player2: String::new(),
        wins1: String::new(),
        wins2: String::new(),
        duration: String::new(),
        decksize1: String::new(),
        decksize2: String::new(),
        deckid1: String::new(),
        deckid2: String::new(),
        ids: HashMap::new(),
        deck1: Vec::new(),
        deck2: Vec::new(),
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
                    let duration = play["seconds"].to_string().parse::<u32>().unwrap();
                    stats.duration = format!(
                        "{}m, {}s",
                        (duration / 60).to_string(),
                        (duration % 60).to_string(),
                    );
                    break;
                }
            }
            if play["id"].is_number() {
                if !stats
                    .ids
                    .contains_key(&(play["id"].to_string().parse::<u32>().unwrap()))
                {
                    stats.ids.insert(
                        (play["id"].to_string().parse::<u32>().unwrap()),
                        play["card"]["name"].to_string(),
                    );
                    /* if play["username"].to_string() == stats.player1 {
                        stats.deck1.push(play["id"].to_string().parse().unwrap());
                    }
                    if play["username"].to_string() == stats.player2 {
                        stats.deck2.push(play["id"].to_string().parse().unwrap());
                    } */
                }
            }
        }
    }
    if let Some(main1) = json["player1"]["main"].as_array() {
        for id in main1 {
            stats.deck1.push(id.to_string().parse().unwrap());
        }
    }
    if let Some(extra1) = json["player1"]["extra"].as_array() {
        for id in extra1 {
            stats.deck1.push(id.to_string().parse().unwrap());
        }
    }
    if let Some(side1) = json["player1"]["side"].as_array() {
        for id in side1 {
            stats.deck1.push(id.to_string().parse().unwrap());
        }
    }
    if let Some(main2) = json["player2"]["main"].as_array() {
        for id in main2 {
            stats.deck2.push(id.to_string().parse().unwrap());
        }
    }
    if let Some(extra2) = json["player2"]["extra"].as_array() {
        for id in extra2 {
            stats.deck2.push(id.to_string().parse().unwrap());
        }
    }
    if let Some(side2) = json["player2"]["side"].as_array() {
        for id in side2 {
            stats.deck2.push(id.to_string().parse().unwrap());
        }
    }
    let mut id_sum = 0;
    for card in stats.deck1.clone().into_iter() {
        id_sum += card;
    }
    stats.deckid1 = (id_sum % 100).to_string();
    id_sum = 0;
    for card in stats.deck2.clone().into_iter() {
        id_sum += card;
    }
    stats.deckid2 = (id_sum % 100).to_string();
    stats.decksize1 = format!(
        "Main: **{}**, Extra: **{}**, Side: **{}**",
        json["player1"]["main_total"].to_string(),
        json["player1"]["extra_total"].to_string(),
        json["player1"]["side_total"].to_string()
    );
    stats.decksize2 = format!(
        "Main: **{}**, Extra: **{}**, Side: **{}**",
        json["player2"]["main_total"].to_string(),
        json["player2"]["extra_total"].to_string(),
        json["player2"]["side_total"].to_string()
    );
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

async fn content(url: &str) -> Result<(Value, u32), fantoccini::error::CmdError> {
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
    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    let response = client
        .execute("return window.replayResponse;", vec![])
        .await?;
    let cards_fingerprint: u32 = client
        .execute("return cards_fingerprint", vec![])
        .await?
        .to_string()
        .parse()
        .unwrap();
    let response_string = response.to_string()[1..response.to_string().len() - 1]
        .replace("\\\\\\\"", "\\\\\"")
        .replace("\\\"", "\"");
    let response: Value = serde_json::from_str(&response_string).unwrap();
    client.close_window().await?;
    Ok((response, cards_fingerprint))
}

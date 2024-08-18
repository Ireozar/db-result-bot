use poise::serenity_prelude::Result;
use reqwest::Error;

pub async fn get_winner(url: String) -> Result<String, Error> {
    let html_content = fetch_html(&url).await?;
    Ok(url)
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

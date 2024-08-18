use fantoccini::{ClientBuilder, Locator};
use poise::serenity_prelude::Result;
use reqwest::Error;

pub async fn get_winner(url: String) -> Result<String, Error> {
    let html_content = fetch_html(&url).await?;
    Ok(url)
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    let winner = content(url).await.unwrap();
    Ok(winner)
}

async fn content(url: &str) -> Result<String, fantoccini::error::CmdError> {
    let mut client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("Failed to create client");
    client.goto(url).await?;
    client.execute("grecaptcha.ready(function() {
            grecaptcha.execute('6LcjdkEgAAAAAKoEsPnPbSdjLkf4bLx68445txKj', {action: 'submit'}).then(function(token) {
                window.recaptchaToken = token;
            });
        });",
    vec![]).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    let token = client
        .execute("return window.recaptchaToken", vec![])
        .await?
        .to_string();
    println!("Extracted reCAPTCHA token: {}", token);
    client.close().await?;
    Ok(token)
}

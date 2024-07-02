use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

const BASE_URL: &str = "https://api.rugcheck.xyz/v1/";

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct PingResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct Risk {
    name: String,
    value: Option<String>,
    description: String,
    score: i32,
    level: String,
}
#[derive(Deserialize, Debug)]
pub struct TokenReportSummary {
    tokenProgram: String,
    tokenType: Option<String>,
    risks: Vec<Risk>,
    score: i32,
}


pub async fn authenticate(client: &Client) -> Result<String, Box<dyn Error>> {
    let url = format!("{}/auth/login/solana", BASE_URL);
    let params = [("publicKey", "9LW5QMP2NM7JBN7HYTvVn4svtcsH1Rp2wGVqn6JCnw6M"), ("signature", "4xafv3tpdKEcLFLh2Z57TDh1YbfVm37TQTEhKrFCRZ7nrEdG8H6v5aa9Coy4Lhfpbxs13cgFDmgN1DiDETT5X7TU")];
    let response = client.post(&url).form(&params).send().await?.json::<AuthResponse>().await?;
    Ok(response.access_token)
}

pub async fn ping(client: &Client, token: &str) -> Result<PingResponse, Box<dyn Error>> {
    let url = format!("{}/ping", BASE_URL);
    let response = client.get(&url).bearer_auth(token).send().await?.json::<PingResponse>().await?;
    Ok(response)
}

pub async fn get_token_report_summary(client: &Client, token_mint: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.rugcheck.xyz/v1/tokens/59YFqj68ZELZ4JSoXEJHw81G8PdVzRgztnaGyVhVBvHS/report/summary");
    println!("2");
    let response = client.get(&url).send().await?;
    let raw_text = response.text().await?;
    Ok(raw_text)
}
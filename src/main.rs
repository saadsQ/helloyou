use std::{error::Error, net::Ipv4Addr};
use reqwest::Client;
use tokio::spawn;

use crate::{utils::measure_latency, rugcheck::{authenticate, ping, get_token_report_summary}};

mod rugcheck;
mod config;
mod client;
mod utils;
mod handlers;
mod tg_bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::load();
    //std::env::set_var("TELOXIDE_TOKEN", &config.bot_token);
    tg_bot::run().await;
   /*  let client = client::connect_telegram(&config).await?;
    
    spawn({
        let client = client.clone();
        let session_path = config.session_path.clone();
        async move {
            handlers::handle_shutdown(client, session_path).await;
        }
    });

    let token = client.request_login_code(&config.phone_number).await?;
    println!("Please check your Telegram and enter the code:");
    let mut code = String::new();
    std::io::stdin().read_line(&mut code)?;
    let code = code.trim().to_string();

    let user = match client.sign_in(&token, &code).await {
        Ok(user) => user,
        Err(err) => {
            println!("Failed to sign in: {}", err);
            return Err(err.into());
        }
    };

    println!("Signed in as {}!", user.first_name());
    client.session().save_to_file(&config.session_path)?;

    //Resolve the username to get user ID and access hash
   //let username = ""; // Replace with the bot's username
   //let result = client.invoke(&ResolveUsername { username: username.into() }).await?;

   //println!("result: {:?}, ", result);

   let session = client.session();

   let connected_dcs = session.get_dcs();
   println!("Trying session and actual DC");
   for dc in connected_dcs {
       let dc_address = format!("{:?}:{}", dc.ipv4, dc.port);
       //let ip = Ipv4Addr::from(dc.ipv4 as u32).to_string();
        println!("User is connected to {} DC to this address : {} ",dc.id,dc_address);  
       
   }



    // Initialize and sync state for all dialogs
   //handlers::handle_new_messages(&client, 2243901023).await?;

   //testting rug
   let client2: Client = Client::new();

/* // Example token mint (replace with real value)
  let token_mint = "59YFqj68ZELZ4JSoXEJHw81G8PdVzRgztnaGyVhVBvHS";
  println!("0");
  // Get token report summary
  match get_token_report_summary(&client2, token_mint).await {
    Ok(summary) => {
        println!("Token Report Summary: {:?}", summary);
    }
    Err(e) => {
        eprintln!("Failed to get token report summary: {}", e);
    }
} */

 */


    Ok(())
}

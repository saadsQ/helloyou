use grammers_client::{Client, Config as GramConfig, InitParams};
use grammers_session::Session;
use std::error::Error;
use std::net::SocketAddr;

pub async fn connect_telegram(config: &crate::config::Config) -> Result<Client, Box<dyn Error>> {
    let server_addr: SocketAddr = config.dc.parse()?;
    let client = Client::connect(GramConfig {
        session: Session::load_file_or_create(&config.session_path)?,
        api_id: config.api_id.parse()?,
        api_hash: config.api_hash.clone(),
        params: InitParams {
            server_addr: Some(server_addr),
            ..InitParams::default()
        },
    }).await?;
    Ok(client)
}
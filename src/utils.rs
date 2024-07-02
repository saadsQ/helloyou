use regex::Regex;
use std::collections::HashSet;
use std::net::ToSocketAddrs;
use std::error::Error;
use tokio::{time::{Duration, Instant, timeout}, io::AsyncWriteExt};

pub fn extract_contract_addresses(message: &str) -> Vec<String> {
    let patterns = vec![
        (r"0x[a-fA-F0-9]{40}", "Ethereum"),
        (r"[A-HJ-NP-Za-km-z1-9]{44}", "Solana"),
        (r"base[A-HJ-NP-Za-km-z1-9]{38}", "Base"),
        (r"UQ[A-HJ-NP-Za-km-z1-9]{48}", "Ton"),
        (r"bnb1[a-z0-9]{38}", "Binance"),
    ];

    let mut addresses: HashSet<String> = HashSet::new();
    for (pattern, chain) in patterns {
        let re = Regex::new(pattern).unwrap();
        for cap in re.captures_iter(message) {
            let address = cap[0].to_string();
            if addresses.insert(address.clone()) {
                println!("Found {} Token: {}", chain, address);
            }
        }
    }
    addresses.into_iter().collect()
}

pub async fn measure_latency(addr: &str) -> Result<Duration, Box<dyn Error>> {
    let socket_addrs: Vec<_> = addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        return Err("No valid socket addresses found".into());
    }

    let start = Instant::now();
    let mut stream = timeout(Duration::from_secs(3), tokio::net::TcpStream::connect(&socket_addrs[0])).await??;
    let duration = start.elapsed();
    stream.shutdown().await?;
    Ok(duration)
}
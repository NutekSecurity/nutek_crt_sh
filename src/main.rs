use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
struct SSLData {
    issuer_ca_id: i64,
    issuer_name: String,
    common_name: String,
    name_value: String,
    id: i64,
    entry_timestamp: String,
    not_before: String,
    not_after: String,
    serial_number: String,
}

fn parse_json(json_str: &str) -> serde_json::Result<Vec<SSLData>> {
    let data: Vec<SSLData> = serde_json::from_str(json_str)?;
    Ok(data)
}


mod cli;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let domain = cli.domain.unwrap_or("".to_string());
    println!("checking domain: {}", domain);
    
    let saving = cli.save.is_some();
    let save = cli.save.unwrap_or("".to_string());
            
    let url = format!("https://crt.sh/?q={}&output=json", domain);
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;
        //.json::<Vec<HashMap<String, String>>>()
        //.await?;
    let jsonize = parse_json(resp.as_str())?;
    let mut subdomains = vec![];
    for i in jsonize.iter() {
        subdomains.push(i.name_value.clone());
        //println!("{}", i.name_value);
    }
    subdomains.sort();
    subdomains.dedup();
    let mut file = File::create(&save)?;
    if &save != "" {
        println!("saving to: {:?}", &save);
    }
    for i in subdomains.iter() {
        println!("{}", i);
        if saving {
            file.write(i.as_bytes())?;
        }
    }
    println!("done. found {} subdomains", subdomains.len());
    Ok(())
}


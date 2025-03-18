use reqwest::blocking::Client;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::env;

const IPFS_API: &str = "http://localhost:5001/api/v0";

fn upload_to_ipfs(file_path: &str) -> Result<String,Box<dyn std::error::Error>> {
    let client = Client::new();
    let  _file = File::open(file_path)?;

    let form = reqwest::blocking::multipart::Form::new()
    .file("file", file_path)?; 

    let response = client
        .post(&format!("{}/add", IPFS_API))
        .multipart(form) 
        .send()?;

    let text = response.text()?;  
    println!("🔍 Raw Response: {}", text);

    let json: Value = serde_json::from_str(&text)?;
    let cid = json["Hash"].as_str().unwrap().to_string();

    println!("Uploaded file to IPFS with CID: {}", cid);
    Ok(cid)
}

fn download_from_ipfs(cid: &str, output_path: &str) -> Result<(),Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:8080/ipfs/{}", cid);


    let response = client.get(&url).send()?;
    let content=response.bytes()?;


    let mut file = File::create(output_path)?;
    file.write_all(&content)?;

    println!("File downloaded and saved as {}", output_path);
    Ok(())
}

fn main()
{
    let args:Vec<String>=env::args().collect();

    if args.len()<3
    {
        println!("Usage:");
        println!(" Upload: cargo run upload <file_path>");
        println!(" Download: cargo run download <cid> <output_path>");
        return;
    }

    let command=&args[1];

    match command.as_str() {
        "upload" => {
            let file_path = &args[2];
            if let Err(e) = upload_to_ipfs(file_path) {
                eprintln!("Error uploading file: {}", e);
            }
        }
        "download" => {
            if args.len() < 4 {
                eprintln!("Missing output file name.");
                return;
            }
            let cid = &args[2];
            let output_file = &args[3];
            if let Err(e) = download_from_ipfs(cid, output_file) {
                eprintln!("Error downloading file: {}", e);
            }
        }
        _ => {
            println!(" Unknown command.");
        }
    }
}
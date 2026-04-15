use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONNECTION, ACCEPT_LANGUAGE, USER_AGENT};
use std::sync::Arc;
use std::sync::Mutex;
use serde_json::{Value};
use reqwest; 
#[derive(Debug, Clone)]

pub struct Mtproxy {
    git_config: String,
    headers: Arc<Mutex<HeaderMap>>,
}

impl Mtproxy{
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36"));

        Self {
            git_config: "https://raw.githubusercontent.com/nellimonix/mtproxy_list".to_string(),
            headers: Arc::new(Mutex::new(headers)),
        }
    }

    pub async fn mtproxy_get(&self) -> Result<Value, Box<dyn std::error::Error>>{
        let url = format!("{}/refs/heads/main/mtproxy.json", self.git_config);
        let  current_headers = self.headers.lock().unwrap().clone();
        let client = reqwest::Client::new();
    
        let response = client
            .get(url)
            .headers(current_headers.clone())
            .send()
            .await?;
        
        let res = response.json().await?;
        Ok(res)
    }
        
    pub async fn show_mtproxy_list(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let configs = self.mtproxy_get().await;
        for config in configs?.as_array().unwrap() {
            let country=config["country"].as_str().expect("REASON").to_string();
            let host=config["host"].as_str().expect("REASON").to_string();
            let port=config["port"].to_string();
            let secret=config["secret"].as_str().expect("REASON").to_string();
            println!("{country} >> tg://proxy?server={host}&port={port}&secret={secret}");
        }
    Ok(().into())
    }
}

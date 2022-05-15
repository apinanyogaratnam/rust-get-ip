use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct IP {
    #[serde(rename = "origin")]
    ip: String,
}

async fn get_ip() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res: IP = client.get("https://httpbin.org/ip").send().await?.json().await?;

    return Ok(res.ip);
}

async fn get_ip_address() -> String {
    let ip = get_ip().await.unwrap();
    return ip;
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ip = get_ip_address().await;
    println!("{}", ip);

    Ok(())
}

// fn main() {
//     yew::start_app::<Model>();
// }

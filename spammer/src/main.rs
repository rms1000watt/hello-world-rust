// ref: https://stackoverflow.com/a/51047786/2330648

use futures::{stream, StreamExt};
use reqwest::Client;
use tokio;

const PARALLEL_REQUESTS: usize = 2;

#[tokio::main]
async fn main() {
    let urls = vec!["https://api.ipify.org"; 2];

    let client = Client::new();

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(b)) => println!("Got {} bytes", b.len()),
                Ok(Err(e)) => eprintln!("Got a reqwest::Error: {}", e),
                Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}

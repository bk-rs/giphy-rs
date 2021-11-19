/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p giphy-api-lite-demo --bin stickers_search 'YOUR_API_KEY' 'dogs'
*/

use std::{env, error};

use futures_lite::future::block_on;
use giphy_api_lite::endpoints::search::{Search, SearchType};
use http_api_isahc_client::{Client as _, IsahcClient};

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let api_key = env::args().nth(1).unwrap();
    let q = env::args().nth(2).unwrap();

    let client = IsahcClient::new()?;

    let stickers_search = Search::new(SearchType::Sticker, api_key, q);

    let ret = client.respond_endpoint(&stickers_search).await?;

    println!("{:?}", ret);

    Ok(())
}

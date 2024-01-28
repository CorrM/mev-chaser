mod utils;

use anyhow::{anyhow, Result};
use ethers::providers::{Provider, Ws};
use std::{env::VarError, sync::Arc};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::utils::env::Env;
use shared::network_streams::{Event, NetworkStreamManager, NetworkStreamManagerBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    if dotenv::dotenv().is_err() {
        return Err(anyhow!("No .env file found"));
    }

    let var_parse: Result<Env, VarError> = Env::new();
    if var_parse.is_err() {
        return Err(anyhow!("Error while parsing .env file: {:?}", var_parse.unwrap_err()));
    }

    let env: Env = var_parse.unwrap();

    // Start async websocket streams
    let ws: Ws = Ws::connect(env.wss_url).await?;
    let ws_provider = Arc::new(Provider::new(ws));
    let mut ns: NetworkStreamManager = NetworkStreamManagerBuilder::new(&ws_provider)
        .watch_new_blocks()
        //.watch_pending_transactions()
        //.watch_log("Sync(uint112,uint112)")
        .build();

    let mut event_receiver: Receiver<Event> = ns.subscribe();
    if let Ok(event) = event_receiver.recv().await {
        println!("{:?}", event);
    }
    //sb.0.spawn(event_handler(provider.clone(), sb.1.clone()));

    while let Some(res) = ns.wait().await {
        println!("{:?}", res);
    }

    Ok(())
}

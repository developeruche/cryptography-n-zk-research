use ethers::{
    contract::{abigen, Contract},
    core::types::ValueOrArray,
    providers::{Provider, StreamExt, Ws},
};
use std::{error::Error, sync::Arc};
use tokio::time::Duration;
use std::time::Instant;



abigen!(
    AggregatorInterface,
    r#"[
        event Transfer(address indexed from, address indexed to, uint value)
    ]"#,
);



const USDT: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = get_client().await;
    let client = Arc::new(client);


    let event = Contract::event_of_type::<TransferFilter>(client)
        .from_block(18103160)
        .address(ValueOrArray::Array(vec![
            USDT.parse()?,
        ]));

    

    let mut last_attempt = Instant::now();

    loop {
        let mut stream = event.subscribe_with_meta().await?.take(2);
        println!("waiting for events .................... :)");

        let elapsed = last_attempt.elapsed();
        if elapsed < Duration::from_secs(1) {
            tokio::time::sleep(Duration::from_secs(1) - elapsed).await;
        }


        match stream.next().await {
            Some(Ok((log, meta))) => {
                println!("{log:?}");
                println!("{meta:?}")
            }
            Some(Err(e)) => println!("Error: {}", e),
            None => (),
        }

        last_attempt = Instant::now();
    }
}




// this function would be used to get an instance of the client
async fn get_client() -> Provider<Ws> {
    Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
        .await
        .unwrap()
}
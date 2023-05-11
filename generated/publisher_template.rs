use std::error::Error;
use futures::StreamExt;

pub async fn start()->Result<(), Box<dyn Error>>{
    let version = "2.0.0";
    println!("generating asycapi version {}", version);
    let nats_url = "nats://localhost:4222";
    let client = async_nats::connect(nats_url).await?;
    let subject = "foo";
    let mut subscription = client.subscribe(subject.to_string()).await?;
    while  let Some(message) = subscription.next().await {
        let message = message;
        println!("Received message: {:?}", message);
    }
   
    Ok(())
}
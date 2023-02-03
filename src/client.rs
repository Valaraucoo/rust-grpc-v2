use tonic::Request;

use hello::{hello_client::HelloClient, HelloRequest};

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloClient::connect("http://[::1]:8080").await?;

    let request = Request::new(HelloRequest {
        name: "Kamil".to_string(),
    });
    let response = client.hello_world(request).await?;
    println!("response: {}", response.into_inner().message);

    Ok(())
}

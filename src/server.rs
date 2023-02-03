use tonic::transport::Server;
use tonic::{Request, Response, Status};

use hello::{HelloRequest, HelloResponse, hello_server::{Hello, HelloServer}};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl Hello for Service {
    async fn hello_world(&self, request : Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let req = request.into_inner();

        let response = HelloResponse {
            message: "Hello, {}!".to_string().replace("{}", &req.name)
        };

        println!("Received request: name={:?}", req.name);
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();

    let service = Service::default();
    Server::builder()
        .add_service(HelloServer::new(service))
        .serve(address)
        .await?;

    Ok(())
}
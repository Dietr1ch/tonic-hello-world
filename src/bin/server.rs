pub mod hello_world {
    use tonic::{Request, Response, Status};
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
    use greeter_server::Greeter;

    #[derive(Debug, Default)]
    pub struct GreeterImpl {}

    #[tonic::async_trait]
    impl Greeter for GreeterImpl {
        async fn say_hello(
            &self,
            request: Request<HelloRequest>, // Accept request of type HelloRequest
        ) -> Result<Response<HelloResponse>, Status> {
            // Return an instance of type HelloResponse
            println!("Got a request: {:?}", request);

            let response = HelloResponse {
                message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
            };

            Ok(Response::new(response)) // Send back our formatted greeting
        }
    }
}

use clap::Parser;
use tonic::transport::Server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Server address
    #[clap(short, long, default_value = "[::1]:50051")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let address = args.address.parse()?;
    println!("Starting up server at '{:?}'", &address);

    let greeter = hello_world::GreeterImpl::default();

    Server::builder()
        .add_service(hello_world::greeter_server::GreeterServer::new(greeter))
        .serve(address)
        .await?;

    Ok(())
}

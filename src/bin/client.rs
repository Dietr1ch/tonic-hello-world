pub mod hello_world {
    tonic::include_proto!("helloworld");
}
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Server address
    #[clap(short, long, default_value = "http://[::1]:50051")]
    server_address: String,

    // Client name
    #[clap(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut client = GreeterClient::connect(args.server_address).await?;

    let request = tonic::Request::new(HelloRequest { name: args.name });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

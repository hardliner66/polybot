use database_server::database_client::DatabaseClient;
use database_server::AddUserRequest;

pub mod database_server {
    tonic::include_proto!("data_access");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DatabaseClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AddUserRequest {
        name: std::env::args().skip(1).next().unwrap().into(),
    });

    let response = client.add_user(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}


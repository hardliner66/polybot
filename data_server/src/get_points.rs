use database_server::database_client::DatabaseClient;
use database_server::GetPointsRequest;

pub mod database_server {
    tonic::include_proto!("data_access");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DatabaseClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetPointsRequest {
        streamer_name: std::env::args().skip(1).next().unwrap().into(),
        viewer_name: std::env::args().skip(2).next().unwrap().into(),
    });

    let response = client.get_points(request).await?;

    println!("RESPONSE={:#?}", response);

    Ok(())
}

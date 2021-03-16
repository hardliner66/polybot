pub mod database {
    tonic::include_proto!("data_access");
}

use database::database_client::DatabaseClient;
use database::{AddPointsRequest, GetPointsRequest, SetPointsRequest};
use tonic::transport::Channel;

pub struct DataServerClient {
    client: database::database_client::DatabaseClient<Channel>,
}

impl DataServerClient {
    pub async fn new(address: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = DatabaseClient::connect(address).await?;

        Ok(DataServerClient { client })
    }

    pub async fn get_points(
        &mut self,
        streamer_name: String,
        viewer_name: String,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetPointsRequest {
            streamer_name,
            viewer_name,
        });

        let response = self.client.get_points(request).await?;

        Ok(response.into_inner().points)
    }

    pub async fn set_points(
        &mut self,
        streamer_name: String,
        viewer_name: String,
        points: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(SetPointsRequest {
            streamer_name,
            viewer_name,
            points,
        });

        let _ = self.client.set_points(request).await?;

        Ok(())
    }

    pub async fn add_points(
        &mut self,
        streamer_name: String,
        viewer_name: String,
        points: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(AddPointsRequest {
            streamer_name,
            viewer_name,
            points,
        });

        let response = self.client.add_points(request).await?;

        Ok(response.into_inner().points)
    }
}

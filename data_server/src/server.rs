use tonic::{transport::Server, Request, Response, Status};

use diesel::r2d2::ConnectionManager;

use database_server::database_server::{Database, DatabaseServer};
use database_server::{AddUserRequest, AddUserResponse, GetUserRequest, GetUserResponse};

use diesel::r2d2;
use diesel::pg::PgConnection;

pub mod database_server {
    tonic::include_proto!("data_access");
}

pub struct MyDatabase
{
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

#[tonic::async_trait]
impl Database for MyDatabase
{
    async fn add_user(
        &self,
        request: Request<AddUserRequest>,
    ) -> Result<Response<AddUserResponse>, Status> {
        println!("Got a request: {:?}", request);
        let client = self.pool.get().unwrap();

        data_server::create_user(&client, &request.get_ref().name);

        let reply = AddUserResponse {
            id: 1,
        };

        Ok(Response::new(reply))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetUserResponse {
            name: "iamhardliner".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");


    let manager = ConnectionManager::new(
        database_url
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    let db = MyDatabase {
        pool
    };

    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(DatabaseServer::new(db))
        .serve(addr)
        .await?;

    Ok(())
}


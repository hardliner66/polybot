use tonic::{transport::Server, Request, Response, Status};

use diesel::r2d2::ConnectionManager;

use data_server::models::{User, UserData};

use common::database::database_server::{Database, DatabaseServer};
use common::database::{
    AddPointsRequest,
    AddPointsResponse,
    GetPointsRequest,
    GetPointsResponse,
    SetPointsRequest,
    SetPointsResponse,
};

use diesel::pg::PgConnection;
use diesel::r2d2;

pub struct MyDatabase {
    pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

fn get_or_add_user(conn: &PgConnection, name: &str) -> Result<User, Status> {
    match data_server::get_user(conn, name) {
        Ok(user) => Ok(user),
        Err(_) => match data_server::create_user(conn, name) {
            Ok(user) => Ok(user),
            Err(_) => Err(Status::internal("Could not get or add user!")),
        },
    }
}

fn get_or_add_user_data(
    conn: &PgConnection,
    streamer: &User,
    viewer: &User,
) -> Result<UserData, Status> {
    match data_server::get_user_data(conn, streamer, viewer) {
        Ok(user_data) => Ok(user_data),
        Err(_) => match data_server::create_user_data(conn, streamer, viewer) {
            Ok(user_data) => Ok(user_data),
            Err(_) => Err(Status::internal("Could not get or add user data!")),
        },
    }
}

#[tonic::async_trait]
impl Database for MyDatabase {
    async fn get_points(
        &self,
        request: Request<GetPointsRequest>,
    ) -> Result<Response<GetPointsResponse>, Status> {
        let request = request.get_ref();

        let conn = self.pool.get().unwrap();

        let streamer = get_or_add_user(&conn, &request.streamer_name)?;
        let viewer = get_or_add_user(&conn, &request.viewer_name)?;

        let user_data = get_or_add_user_data(&conn, &streamer, &viewer)?;

        Ok(Response::new(GetPointsResponse {
            points: user_data.points,
        }))
    }

    async fn set_points(
        &self,
        request: Request<SetPointsRequest>,
    ) -> Result<Response<SetPointsResponse>, Status> {
        let request = request.get_ref();

        let conn = self.pool.get().unwrap();

        let streamer = get_or_add_user(&conn, &request.streamer_name)?;
        let viewer = get_or_add_user(&conn, &request.viewer_name)?;

        data_server::set_points(&conn, &streamer, &viewer, request.points)
            .map_err(|_| Status::internal("oops"))?;

        Ok(Response::new(SetPointsResponse {}))
    }

    async fn add_points(
        &self,
        request: Request<AddPointsRequest>,
    ) -> Result<Response<AddPointsResponse>, Status> {
        let request = request.get_ref();

        let conn = self.pool.get().unwrap();

        let streamer = get_or_add_user(&conn, &request.streamer_name)?;
        let viewer = get_or_add_user(&conn, &request.viewer_name)?;

        let new_points = data_server::add_points(&conn, &streamer, &viewer, request.points)
            .map_err(|_| Status::internal("oops"))?;

        Ok(Response::new(AddPointsResponse { points: new_points }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).unwrap();

    let db = MyDatabase { pool };

    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(DatabaseServer::new(db))
        .serve(addr)
        .await?;

    Ok(())
}

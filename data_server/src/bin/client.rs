use structopt::{StructOpt, clap::AppSettings};

use database_server::database_client::DatabaseClient;
use database_server::AddPointsRequest;
use database_server::GetPointsRequest;
use database_server::SetPointsRequest;

pub mod database_server {
    tonic::include_proto!("data_access");
}

#[derive(StructOpt)]
#[structopt(global_setting = AppSettings::AllowNegativeNumbers)]
struct Opt {
    #[structopt(default_value = "http://[::1]:50051", short, long)]
    /// address of the server
    server: String,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    /// everything channel related
    Channel {
        /// the name of the channel
        channel: String,
        #[structopt(subcommand)]
        command: ChannelCommand,
    },
}

#[derive(StructOpt)]
enum ChannelCommand {
    /// everything viewer related
    Viewer {
        /// the name of the viewer
        viewer: String,
        #[structopt(subcommand)]
        command: Option<ViewerCommand>,
    },
}

#[derive(StructOpt)]
enum ViewerCommand {
    /// add the specified amount of points to a viewer
    AddPoints {
        points: i32,
    },
    /// set the points of a viewer to the specified amount
    SetPoints {
        points: i32,
    },
    /// get the points of a viewer (default if no command is specified)
    GetPoints,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opt { server, command } = Opt::from_args();
    let mut client = DatabaseClient::connect(server).await?;

    match command {
        Command::Channel { channel, command } => match command {
            ChannelCommand::Viewer { viewer, command } => {
                match command.unwrap_or(ViewerCommand::GetPoints) {
                    ViewerCommand::SetPoints { points } => {
                        let request = tonic::Request::new(SetPointsRequest {
                            streamer_name: channel,
                            viewer_name: viewer,
                            points,
                        });

                        let response = client.set_points(request).await?;

                        println!("RESPONSE={:#?}", response);
                    }
                    ViewerCommand::AddPoints { points } => {
                        let request = tonic::Request::new(AddPointsRequest {
                            streamer_name: channel,
                            viewer_name: viewer,
                            points,
                        });

                        let response = client.add_points(request).await?;

                        println!("RESPONSE={:#?}", response);
                    }
                    ViewerCommand::GetPoints => {
                        let request = tonic::Request::new(GetPointsRequest {
                            streamer_name: channel,
                            viewer_name: viewer,
                        });

                        let response = client.get_points(request).await?;

                        println!("RESPONSE={:#?}", response);
                    }
                }
            }
        },
    }

    Ok(())
}

use structopt::{StructOpt, clap::AppSettings};

use common::DataServerClient;

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
        points: i64,
    },
    /// set the points of a viewer to the specified amount
    SetPoints {
        points: i64,
    },
    /// get the points of a viewer (default if no command is specified)
    GetPoints,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opt { server, command } = Opt::from_args();
    let mut client = DataServerClient::new(server).await?;

    match command {
        Command::Channel { channel, command } => match command {
            ChannelCommand::Viewer { viewer, command } => {
                match command.unwrap_or(ViewerCommand::GetPoints) {
                    ViewerCommand::SetPoints { points } => {
                        client.set_points(channel, viewer, points).await?;
                        println!("Succesfully set points!");
                    }
                    ViewerCommand::AddPoints { points } => {
                        let new_points = client.add_points(channel.clone(), viewer.clone(), points).await?;
                        println!("Viewer {} now has {} points in channel {}", viewer, new_points, channel);
                    }
                    ViewerCommand::GetPoints => {
                        let new_points = client.get_points(channel.clone(), viewer.clone()).await?;
                        println!("Viewer {} now has {} points in channel {}", viewer, new_points, channel);
                    }
                }
            }
        },
    }

    Ok(())
}

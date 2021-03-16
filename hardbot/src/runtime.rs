use std::collections::HashMap;

use common::DataServerClient;

const DATA_FILE: &str = "data.toml";

pub struct Chatter {
    pub remaining_ticks: i64,
}

impl Default for Chatter {
    fn default() -> Self {
        Chatter { remaining_ticks: 0 }
    }
}

pub struct Data {
    pub chatters: HashMap<String, Chatter>,
    pub data_server_client: DataServerClient,
}

impl Data {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            chatters: HashMap::new(),
            data_server_client: DataServerClient::new("http://[::1]:50051".to_string()).await?,
        })
    }
}

pub type StringCommands = HashMap<String, String>;

pub struct Runtime {
    pub command_prefix: char,
    pub shoutout_short: char,
    pub config: crate::config::Config,
    pub commands: StringCommands,
    pub data: Data,
}

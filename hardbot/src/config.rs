use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::runtime::{Data, Runtime};

pub const DEFAULT_STREAMER_NAME: &str = "iamhardliner";
pub const DEFAULT_COMMAND_PREFIX: char = '$';
pub const DEFAULT_SHOUTOUT_SHORT: char = '#';

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Points {
    pub ticks_per_message: i64,
    pub points_per_tick: i64,
    pub tick_speed: u64,
    pub steal_chance: f32,
    pub steal_min: u64,
    pub steal_max: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoCommands {
    pub time: f32,
    #[serde(default)]
    pub commands: Vec<String>,
}

impl Default for AutoCommands {
    fn default() -> Self {
        AutoCommands {
            time: 600.0,
            commands: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Group {
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    pub save_time: f32,
    pub hype_emote: String,
    pub message_timeout: Option<u64>,
    #[serde(default)]
    pub ignored_users: Vec<String>,
    #[serde(default)]
    pub shoutout_short: Option<char>,
    #[serde(default)]
    pub command_prefix: Option<char>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub aliases: Vec<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrCommand {
    Text(String),
    Cmd(Command),
}

impl ToString for StringOrCommand {
    fn to_string(&self) -> String {
        match self {
            StringOrCommand::Text(txt) => txt.clone(),
            StringOrCommand::Cmd(cmd) => cmd.value.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub points: Points,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(default)]
    pub commands: HashMap<String, StringOrCommand>,
    #[serde(default)]
    pub auto_commands: AutoCommands,
    #[serde(default)]
    pub groups: HashMap<String, Group>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Config {
        std::fs::read_to_string(path)
            .iter()
            .map(|s| toml::from_str(&s).unwrap())
            .next()
            .unwrap()
    }

    pub async fn to_runtime(self) -> Result<crate::runtime::Runtime, Box<dyn std::error::Error>> {
        let command_prefix = self.general.command_prefix.unwrap_or(DEFAULT_COMMAND_PREFIX);
        let shoutout_short = self.general.shoutout_short.unwrap_or(DEFAULT_SHOUTOUT_SHORT);
        let mut commands = HashMap::new();
        for (name, value) in &self.commands {
            let mut value = value.to_owned();
            for (variable_name, variable_value) in &self.variables {
                match &mut value {
                    StringOrCommand::Text(value) => {
                        *value = value.replace(&format!("{}{}", command_prefix, variable_name), &variable_value);
                    }
                    StringOrCommand::Cmd(cmd) => {
                        cmd.value = cmd
                            .value
                            .replace(&format!("{}{}", command_prefix, variable_name), &variable_value);
                    }
                }
            }
            let text = value.to_string();
            commands.insert(format!("{}", name), text.clone());
            if let StringOrCommand::Cmd(cmd) = &value {
                for name in &cmd.aliases {
                    if !commands.contains_key(name) {
                        commands.insert(name.to_owned(), text.clone());
                    }
                }
            }
        }

        let rt = Runtime {
            command_prefix,
            shoutout_short,
            commands,
            data: Data::new().await?,
            config: self,
        };

        Ok(rt)
    }
}

const DEFAULT_CONFIG: &str = include_str!("../defaults.toml");
impl Default for Config {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG).unwrap()
    }
}

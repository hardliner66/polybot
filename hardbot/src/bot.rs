use crate::config::{DEFAULT_STREAMER_NAME,Config};
use crate::runtime::{StringCommands, Data, Runtime};
use strum_macros::{EnumString, EnumIter, ToString};
use strum::IntoEnumIterator;

pub struct Handler {
    allow_list: Option<Vec<String>>,
}

fn get_name_from_string(s: &str) -> String {
    s.trim().replace("@", "")
}

#[derive(Debug, PartialEq, EnumString, EnumIter, ToString)]
#[strum(serialize_all = "snake_case")]
pub enum Command {
    Calc,
    // Hype,
    Points,
    // Mod,
    // Top,
    // Steal,
    // Give,
    Lurk,
    Unlurk,
    Hi,
    Shoutout,
    Commands,
}

impl Command {
    async fn execute(&self, runtime: &mut Runtime, name: &str, msg: &str) -> Option<String> {
        use Command::*;
        match self {
            Calc => {
                self.calc(msg).await
            },
            Points => {
                self.points(&mut runtime.data, name, msg).await
            },
            // Top => {},
            // Give => {},
            Lurk => {
                self.lurk(name).await
            },
            Unlurk => {
                self.unlurk(name).await
            },
            Hi => {
                self.hi(name).await
            },
            Shoutout => {
                self.shoutout(msg).await
            },
            Commands => {
                self.commands(runtime.command_prefix, &runtime.commands).await
            },
        }
    }

    async fn calc(&self, msg: &str) -> Option<String> {
        let mut ns = fasteval::EmptyNamespace;
        match fasteval::ez_eval(&msg[4..], &mut ns) {
            Ok(v) => Some(v.to_string()),
            Err(e) => Some(e.to_string()),
        }
    }

    async fn points(&self, data: &mut Data, name: &str, msg: &str) -> Option<String> {
        let message = get_name_from_string(&msg[6..]);
        let name = if message.is_empty() { name } else { &message };
        let response = format!(
            "@{} currently has {} points!",
            name,
            data.data_server_client
                .get_points(DEFAULT_STREAMER_NAME.to_string(), name.to_string())
                .await
                .unwrap()
        );
        Some(response)
    }

    // async fn give(&self, config: &mut Config, string_commands: &StringCommands, data: &mut Data, name: &str, msg: &str) -> Option<String> {
    // }
    
    async fn lurk(&self, name: &str) -> Option<String> {
        Some(format!("Have fun lurking @{}! iamhar2Bob", name))
    }

    async fn unlurk(&self, name: &str) -> Option<String> {
        Some(format!("Welcome back from lurking @{}! iamhar2Bob", name))
    }

    async fn hi(&self, name: &str) -> Option<String> {
        Some(format!("Hello, {}!", name))
    }

    async fn shoutout(&self, msg: &str) -> Option<String> {
        let parts = msg.split(" ").collect::<Vec<_>>();
        if let Some(part) = parts.get(1) {
            let name = get_name_from_string(part);
            Some(format!(
                "Shoutout to @{}. You can check out their stream at https://www.twitch.tv/{}",
                name, name
            ))
        } else {
            None
        }
    }

    async fn commands(&self, command_prefix: char, string_commands: &StringCommands) -> Option<String> {
        let mut handlers = Command::iter().map(|c| Command::to_string(&c)).collect::<Vec<_>>();
        let mut commands = string_commands.keys().cloned().collect::<Vec<_>>();
        commands.append(&mut handlers);
        commands.iter_mut().for_each(|c| c.insert(0, command_prefix));
        let response = commands.join(" | ");
        Some(response)
    }
}

pub struct Bot {
    pub runtime: Runtime,
}

impl Bot {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Bot {
            runtime: config.to_runtime().await?,
        })
    }

    pub async fn handle_message(&mut self, name: &str, message: &str) -> Option<String> {
        println!("{}: {}", name, message);
        let name = name.trim().to_lowercase();
        let mut message = message.trim().to_lowercase();

        if message.starts_with(self.runtime.shoutout_short) {
            message = format!("{}so {}", self.runtime.command_prefix, &message[1..]);
        }

        if !message.starts_with(self.runtime.command_prefix) {
            (*self
                .runtime
                .data
                .chatters
                .entry(name.to_owned())
                .or_insert(Default::default()))
            .remaining_ticks = self.runtime.config.points.ticks_per_message;
            None
        } else {
            let message = message
                .chars()
                .skip_while(|&c| c == self.runtime.command_prefix)
                .collect::<String>();
            

            // let allowed = if let Some(allow_list) = &handler.allow_list {
            //     allow_list.contains(&format!("@{}", name))
            //         || self
            //             .runtime
            //             .config
            //             .groups
            //             .iter()
            //             .filter(|(_, g)| g.users.contains(&name.to_owned()))
            //             .any(|(name, _)| {
            //                 allow_list.contains(&format!("#{}", name))
            //             })
            // } else {
            //     true
            // };

            match self.runtime.commands.get(&message).cloned() {
                Some(response) => Some(response),
                _ => match message.split(' ').next() {
                    Some(user_command) => {
                        let command_result: Result<Command, _> = user_command.parse();
                        match command_result {
                            Ok(command) => {
                                command.execute(&mut self.runtime, &name, &message).await
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
        }
    }
}

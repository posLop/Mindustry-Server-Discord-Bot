use std::time::Duration;
use std::net::TcpStream;
use serenity::prelude::{TypeMapKey};
use std::{str};

pub struct TcpSock {
    pub stream: TcpStream,
}

impl TcpSock {
    pub fn new(ip: String, port: String) -> std::io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).expect("Tcp connection fail");
        stream.set_read_timeout(Some(Duration::from_millis(25)))?;
        println!("Socket Connected!!");
        Ok(TcpSock { stream })
    }
}

impl TypeMapKey for TcpSock {
    type Value = TcpSock;
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Config {
    pub discord_settings: DiscordSettings,
    pub admin_roles: PermissionRoles,
    pub console: AdminConsole,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DiscordSettings {
    pub discord_token: String,
    pub ip: String,
    pub port: String,
    pub prefix: String,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct PermissionRoles {
    pub owners: Vec<String>,
    pub admins: Vec<String>
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct AdminConsole {
    pub commands_whitelist: bool,
    pub commands: Vec<String>,
}


impl TypeMapKey for Config {
    type Value = Config;
}
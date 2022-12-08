use crate::structs::*;
use std::str::FromStr;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, BufRead};
use serenity::model::prelude::{Message, RoleId};
use serenity::prelude::{Context};
use std::{str};
use serenity::prelude::SerenityError;
use indoc::indoc;



pub fn cons_rw(sock: &TcpSock, input: &str) -> String {

    let mut output = String::new();

    let mut writer = std::io::BufWriter::new(sock.stream.try_clone().unwrap());
    let mut reader = std::io::BufReader::new(sock.stream.try_clone().unwrap());

    writer.write((input.to_owned() + "\n").as_bytes()).unwrap();
    writer.flush().expect("flush failed");
    
    loop {
        match reader.read_line(&mut output) {
            Ok(t) => t,
            Err(_) => break(),
        };
    }
    
    output = String::from_utf8(strip_ansi_escapes::strip(&output).unwrap()).unwrap();
    output.truncate(4000);
    output
}


pub async fn init_conf() -> Config {

    let mut toml_file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("config.toml")
    .unwrap_or_else(|_e| toml_make());
    
    let mut toml_str = String::new();

    toml_file.read_to_string(&mut toml_str).unwrap();
    
    let config: Config = toml::from_str(&toml_str).expect("unable to fill Config struct");

    config
}


fn toml_make() -> File {
println!("initializing config");
let mut toml_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("config.toml")
        .unwrap();

    let fill_conf = indoc! {r#"
[discord_settings]

# Discord bot token
discord_token = ""

# Ip of the mindustry server
ip = "localhost"


# Port of the mindustry server socket
# Run 'config socketInputPort' in the mindustry console to find this port
port = "6859"

# Prefix used to call commands
# Can be any word letter or symbol
prefix = ";"


# These are the role ids needed in order to use the console command
# If an invalid role is used it will be ignored
# If all the roles are invalid or the list is empty the setting will be ignored
[admin_roles]

# people with roles ids in the owner setting can use all console commands
# if left empty anyone can use any of the commands
owners = ["738543444322156574", "822523680391037009"]

# list of admin roles
admins = []

# this controls which commands admins have access too
[console]

# whether the command list is a whitelist or a blacklist
# true = whitelist
# false = blacklist
commands_whitelist = true

# which commands are whitelisted/blacklisted to admins
commands = ["config"]
"#};

    
    toml_file.write(&fill_conf.as_bytes()).expect("Unable to write to new file");
    toml_file.flush().unwrap();
    toml_file.rewind().unwrap();
    toml_file
}


pub async fn check_role(ctx: &Context, msg: &Message, roles: &Vec<String>) -> Result<bool, SerenityError> {    
    let mut invalid_roles = 0;

    for v_id in roles {
    let u_id = match u64::from_str(&v_id) {
        Ok(n) => n,
        Err(_e) => 
        {
            invalid_roles += 1;
            continue
        },
    };

    let check = msg.author.has_role(ctx, msg.guild_id.unwrap(), RoleId::from(u_id)).await?;
    if check {
        return Ok(check);
        }
    }

    if invalid_roles == roles.len() {
        return Ok(true)
    }

    Ok(false)
}

pub fn is_command(command: String, command_vec: &Vec<String>) -> bool {
    for r in command_vec {
        if r == &command {
            return true;
        }
    }
    false
}

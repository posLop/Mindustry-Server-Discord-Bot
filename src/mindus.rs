use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, BufRead};
use std::time::Duration;
use std::net::TcpStream;
use serenity::prelude::TypeMapKey;
use std::str;
use indoc::indoc;

pub struct TcpSock {
    pub stream: TcpStream,
}

impl TcpSock {
    pub fn new(ip: String, port: String) -> std::io::Result<Self> {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).expect("Tcp connection fail");
        stream.set_read_timeout(Some(Duration::from_millis(200)))?;

        Ok(TcpSock { stream })
    }
}

impl TypeMapKey for TcpSock {
    type Value = TcpSock;
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub discord_token: String,
    pub ip: String,
    pub port: String,
    pub prefix: String,
    pub roles: Roles
}


#[derive(serde::Deserialize, serde::Serialize)]
pub struct Roles {
    pub auth: Vec<String>,
    pub cons: Vec<String>
}

impl TypeMapKey for Config {
    type Value = Config;
}

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
    println!("{}", output);
    output = String::from_utf8(strip_ansi_escapes::strip(&output).unwrap()).unwrap();
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
    
    println!("{}", toml_str);

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

    let fill_conf = 
    indoc! {r#"
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
    
    # These are the roles needed in order to use the associated command
    # If an invalid role is used it will be ignored.  If all the roles are invalid or the list is empty then anyone can use the command
    [roles]
    # Auth command
    auth = [""]
    
    # console command
    cons = ["738543444322156574", "822523680391037009"]
    "#};

    // let fill_conf = Config {
    //     discord_token: String::from(""),
    //     ip: String::from("localhost"),
    //     port: String::from("6859"),
    //     prefix: String::from(";"),
    //     roles: Roles {
    //         auth: vec![String::from(""), String::from("")],
    //         cons: vec![String::from("")]
    //     }
    // };

    toml_file.write(fill_conf.as_bytes()).expect("Unable to write to new file");
    toml_file.flush().unwrap();
    toml_file.rewind().unwrap();
    toml_file
}
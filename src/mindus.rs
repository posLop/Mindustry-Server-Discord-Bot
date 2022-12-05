use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, BufRead};
use std::time::Duration;
use std::net::TcpStream;
use serenity::prelude::TypeMapKey;
use std::str;

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

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub trigger: char,
    pub roles: String
}

pub fn init_conf() -> Config {

    let mut toml_file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("config.toml")
    .unwrap_or_else(|_e| toml_make());
    // .unwrap_or(toml_make());
    
    let mut toml_str = String::new();

    toml_file.read_to_string(&mut toml_str).unwrap();
    
    println!("{}", toml_str);

    let config: Config = toml::from_str(&toml_str).expect("unable to fill Config struct");

    // let mut conf_vec: Vec<u8> = vec![];
    // toml_file.read_to_end(&mut conf_vec).expect("unable to read toml to string");
    // let toml_str = str::from_utf8(&conf_vec).expect("unable to convert to string");
    // println!("{}", toml_str);
    // let config: Config = toml::from_str(toml_str).expect("unable to fill Config struct");
    
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

    let fill_conf = Config {
        ip: String::from("localhost"),
        port: String::from("6859"),
        trigger: ';',
        roles: String::new()
    };

    toml_file.write(toml::to_string(&fill_conf).unwrap().as_bytes()).expect("Unable to write to new file");
    toml_file.flush().unwrap();
    toml_file.rewind().unwrap();
    toml_file
}
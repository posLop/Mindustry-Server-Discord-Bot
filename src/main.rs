use std::io::{Write, BufRead};
use std::time::Duration;
use std::{env, io, clone};
use serenity::async_trait;
use serenity::futures::io::{BufReader, BufWriter};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult, Args};
use dotenv::dotenv;
use std::net::TcpStream;



struct TcpSock {
    stream: TcpStream,
    reader: std::io::BufReader<TcpStream>,
    writer: std::io::BufWriter<TcpStream>
}


impl TypeMapKey for TcpSock {
    type Value = TcpSock;
}

impl TcpSock {
    pub fn new() -> std::io::Result<Self> {
        let stream = TcpStream::connect("localhost:6859").expect("Tcp connection fail");
        stream.set_read_timeout(Some(Duration::from_millis(200)))?;
        let mut reader = std::io::BufReader::new(stream.try_clone()?);
        let mut writer = std::io::BufWriter::new(stream.try_clone()?);

        Ok(TcpSock { stream, reader, writer })
    }
}


#[group]
#[commands(ping, pong, send)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    
    let sock = TcpSock::new().unwrap();
    dotenv().ok();
        
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<TcpSock>(sock);
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}


#[command]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Ping!").await?;

    Ok(())
}

#[command]
async fn send(ctx: &Context, msg: &Message) -> CommandResult {

    let input = msg.content.strip_prefix(";send ").to_owned();
    let output = &mut String::new();
    if input == Option::None {
        msg.reply(ctx, "Not enough Parameters").await?;
        return Ok(());
    }

    let data = ctx.data.read().await;
    let sock = data.get::<TcpSock>().unwrap();

    
    let mut writer = std::io::BufWriter::new(sock.stream.try_clone()?);
    let mut reader = std::io::BufReader::new(sock.stream.try_clone()?);

    writer.write((input.unwrap().to_owned() + "\n").as_bytes())?;
    writer.flush().expect("flush failed");
    
    loop {
        match reader.read_line(output) {
            Ok(t) => t,
            Err(_) => break(),
        };
    }
    println!("{}", output);
    msg.reply(ctx, format!("```ansi\n{}\n```", output)).await?;
    Ok(())
}


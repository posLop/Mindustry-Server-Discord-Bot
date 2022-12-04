mod mindus;
use crate::mindus::*;
use std::io::{Write, BufRead};
use std::{env};
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use dotenv::dotenv;

#[group]
#[commands(ping, pong, console)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    
    let conf = init_conf();

    let sock = TcpSock::new(conf.ip, conf.port).unwrap();

    dotenv().ok();
        
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(conf.trigger)) 
        .group(&GENERAL_GROUP);

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
async fn console(ctx: &Context, msg: &Message) -> CommandResult {

    let input = msg.content.strip_prefix(";console ").to_owned();
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


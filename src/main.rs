mod mindus;
use crate::mindus::*;
// use std::{env};
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};



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

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(conf.trigger)
            .case_insensitivity(true)) 
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&conf.discord_token, intents)
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

    if input == Option::None {
        msg.reply(ctx, "Not enough Parameters").await?;
        return Ok(());
    }

    let data = ctx.data.read().await;

    let sock = data.get::<TcpSock>().unwrap();
    
    msg.reply(ctx, format!("```\n{}\n```", cons_rw(sock, &input.unwrap()))).await?;
    Ok(())
}


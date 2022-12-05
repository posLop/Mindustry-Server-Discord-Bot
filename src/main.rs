mod mindus;
use crate::mindus::*;
// use std::{env};
use serenity::async_trait;
use serenity::model::prelude::{RoleId, UserId};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group, help, hook};
use serenity::framework::standard::{StandardFramework, CommandResult, Args, HelpOptions, CommandGroup, help_commands};
use serenity::utils::Color;
use std::collections::HashSet;
use std::error::Error;
use std::io::Stderr;
use std::str::FromStr;

#[group]
#[commands(ping, pong, console, git, discord)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[tokio::main]
async fn main() {
    
    let conf = init_conf();

    let sock = TcpSock::new(conf.ip.clone(), conf.port.clone()).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(conf.prefix.clone())
            .case_insensitivity(true)) 
            .help(&MY_HELP)
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
        data.insert::<Config>(conf)
        }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}


#[command]
#[help_available(false)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}


#[command]
#[help_available(false)]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Ping!").await?;

    Ok(())
}

#[command]
#[aliases("c", "cons")]
#[description("Send a command to the mindustry server console")] 
#[example("c status")]
async fn console(ctx: &Context, msg: &Message) -> CommandResult {
    let input = msg.content.strip_prefix(";console ").to_owned();

    if input == Option::None {
        msg.reply(ctx, "Not enough Parameters").await?;
        return Ok(());
    }

    let data = ctx.data.read().await;

    let sock = data.get::<TcpSock>().unwrap();
    // let conf = data.get::<Config>().unwrap();
    
    // if !check_role(ctx, msg, conf).await.unwrap_or_else(|e| false) {
    //     // msg.channel_id.say(ctx, "You do not have permission to use this command").await?;
    //     msg.channel_id.send_message(ctx, |m| {
    //         m.content("test")
    //             .embed(|e| e
    //                 .title("No Permissions")
    //                 .description("You do not have permission to use this command")
    //                 .color(Color::RED))
    //     }).await?;
    //     return Ok(());
    // }

    msg.reply(ctx, format!("```\n{}\n```", cons_rw(sock, &input.unwrap()))).await?;
    Ok(())
}

#[command]
async fn git(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://mintyserver.net/git/poslop/Mindustry-Server-Discord-Bot").await?;
    Ok(())
}

#[command]
async fn discord(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://discord.gg/sRKCKQAdU4").await?;
    Ok(())
}

// async fn check_role(ctx: &Context, msg: &Message, conf: &Config) -> Result<bool, SerenityError> {
//     let id = RoleId::from(u64::from_str(&conf.roles.cons))?;
//     let check = msg.author.has_role(ctx, msg.guild_id.unwrap(), id).await?;
//     Ok(check)
// }
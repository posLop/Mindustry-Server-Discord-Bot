mod mindus;
use crate::mindus::*;
use serenity::async_trait;
use serenity::model::prelude::{UserId, RoleId};
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group, help, hook};
use serenity::framework::standard::{StandardFramework, CommandResult, Args, HelpOptions, CommandGroup, help_commands};
use serenity::utils::Color;
use std::collections::HashSet;
use std::str::FromStr;

#[group]
#[commands(ping, pong, console, git, discord, auth)]
struct Commands;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[help]
#[strikethrough_commands_tip_in_guild("")] 
#[max_levenshtein_distance(1)]
#[available_text("")] 
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
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[tokio::main]
async fn main() {
    

    let conf = init_conf().await;

    let sock = TcpSock::new(conf.ip.clone(), conf.port.clone()).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(conf.prefix.clone())
            .case_insensitivity(true))
            .unrecognised_command(unknown_command) 
            .help(&MY_HELP)
            .group(&COMMANDS_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&conf.discord_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Discord Bot Connected!!");

    {
        let mut data = client.data.write().await;
        data.insert::<TcpSock>(sock);
        data.insert::<Config>(conf)
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?} \n Check that there is a bot token set in config", why);
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
#[example("status")]
#[min_args(1)]
async fn console(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let data = ctx.data.read().await;

    let sock = data.get::<TcpSock>().unwrap();
    let conf = data.get::<Config>().unwrap();

    
    if !check_role(ctx, msg, &conf.roles.cons).await.unwrap_or_else(|_e| true) {
        msg.channel_id.send_message(ctx, |m| {
            m.content("")
                .embed(|e| e
                    .title("No Permissions")
                    .description("You do not have permission to use this command")
                    .color(Color::RED))
        }).await?;
        return Ok(());
    }

    msg.channel_id.send_message(ctx, |m| {
        m.content("")
            .embed(|e| e
                .title("Console")
                .description(cons_rw(sock, args.message()))
                .color(Color::ROSEWATER)
            )
    }).await?;
    Ok(())
}

#[command]
#[num_args(1)]
#[aliases("a")]
#[description("Give youself permissions to build in the mindustry server")] 
#[example("ErkDog")]
async fn auth(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let data = ctx.data.read().await;

    let sock = data.get::<TcpSock>().unwrap();
    let conf = data.get::<Config>().unwrap();

    
    if !check_role(ctx, msg, &conf.roles.auth).await.unwrap_or_else(|_e| true) {
        msg.channel_id.send_message(ctx, |m| {
            m.content("")
                .embed(|e| e
                    .title("No Permissions")
                    .description("You do not have permission to use this command")
                    .color(Color::RED))
        }).await?;
        return Ok(());
    }

    msg.channel_id.send_message(ctx, |m| {
        m.content("")
            .embed(|e| e
                .title("Console")
                .description(cons_rw(sock, &format!("auth add {}", args.message())))
                .color(Color::ROSEWATER)
            )
    }).await?;
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

async fn check_role(ctx: &Context, msg: &Message, roles: &Vec<String>) -> Result<bool, SerenityError> {
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
        return Ok(check)
        }
    }

    if invalid_roles == roles.len() {
        return Ok(true)
    }

    Ok(false)
}
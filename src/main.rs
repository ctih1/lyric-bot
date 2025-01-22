mod lyrics;

use core::time;
use std::env;

use dotenv::dotenv;

use poise::serenity_prelude as serenity;
use poise::serenity_prelude::*;

// use serenity::all::standard::CommandResult;
// use serenity::model::gateway::Ready;
// use serenity::{async_trait, gateway};
// use serenity::model::channel::Message;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    println!("{token}");

    lyrics::get_lyrics("kanye-west", "ultralight-beam");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();


    let client = ClientBuilder::new(token,intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
mod lyrics;
#[macro_use]
extern crate slugify;
use std::env;
use std::collections::HashMap;

use dotenv::dotenv;

use poise::{serenity_prelude as serenity, CreateReply};
use poise::serenity_prelude::*;

// use serenity::all::standard::CommandResult;
// use serenity::model::gateway::Ready;
// use serenity::{async_trait, gateway};
// use serenity::model::channel::Message;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn lyrics(
    ctx:Context<'_>,
    #[description="User whose song to pick"] user: Option<serenity::User>
) -> Result<(), Error> {
    let activities: HashMap<UserId, Presence> = ctx.guild().as_ref().unwrap().presences.clone();
    let activity_data: Option<&Vec<Activity>> = activities.get(&ctx.author().id).map(|presence | &presence.activities);
    let mut song_name:String = String::new();
    let mut artist_name:String = String::new(); 

    if let Some(activities) = activity_data {
        for activity in activities {
            if activity.name.to_lowercase() != "spotify" { continue; }
            
            song_name = activity.details.as_ref().unwrap_or(&String::from("Unknown")).clone();
            artist_name = activity.state.as_ref().unwrap_or(&String::from("Unknown")).clone();

            break;
        }
    } else {
        println!("No activities!");
    }


    let lyrics:String = lyrics::get_lyrics(&artist_name, &song_name).await;

    println!("{}",lyrics);

    let embed = CreateEmbed::new()
        .title(format!("Lyrics for {} by {}", song_name, artist_name))
        .description(lyrics);


    let response = poise::CreateReply::default().embed(embed);

    ctx.send(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    println!("{token}");    

    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_PRESENCES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), lyrics()],
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
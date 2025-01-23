mod lyrics;
mod translation;
#[macro_use]
extern crate slugify;
use std::env;
use std::collections::HashMap;

use dotenv::dotenv;

use poise::serenity_prelude as serenity;
use poise::serenity_prelude::*;
use ureq::rustls::unbuffered::TransmitTlsData;

struct Data {}
struct Bot;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn lyrics(
    ctx:Context<'_>,
    #[description="User who's song to pick"] user: Option<serenity::User>,
    #[description="Enable translation"] translate: Option<bool>
) -> Result<(), Error> {
    let activities: HashMap<UserId, Presence> = ctx.guild().as_ref().unwrap().presences.clone();
    let target_user: &User = user.as_ref().unwrap_or(ctx.author());
    let activity_data: Option<&Vec<Activity>> = activities.get(&target_user.id).map(|presence | &presence.activities);
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


    let lyric_data:Result<lyrics::Lrclib, String> = lyrics::get_lyrics(&artist_name, &song_name).await;

    if lyric_data.is_err() {
        ctx.say(format!("Could not find lyrics for song {} by {}",song_name,artist_name)).await;
        return Ok(())
    }

    let mut lyrics:String = lyric_data.unwrap().plainLyrics;
    
    if translate.unwrap_or(false) {
        lyrics = translation::translate(&lyrics).await.translations.first().unwrap().text.clone();
    }

    lyrics = lyrics.replace("#",""); // some songs keep having a # in their lyrics, which causes discord to start seizing

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
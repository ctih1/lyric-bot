

use slugify::slugify;
use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use reqwest;

#[derive(Deserialize)]
pub struct Lrclib {
    id: i32,
    name: String,
    trackName:String,
    artistName:String,
    albumName:String,
    duration: f32,
    instrumental: bool,
    pub plainLyrics: String,
    syncedLyrics: Option<String>
}

pub async fn get_lyrics(artist_name: &str, song_name: &str) -> Result<Lrclib, String> {
    let request:reqwest::Response  = reqwest::get(
        format!("https://lrclib.net/api/get?artist_name={}&track_name={}",slugify!(artist_name,separator="+"),slugify!(song_name,separator="+"))
    )
        .await
        .unwrap();
    
    if !request.status().is_success() {
        return Err(request.status().to_string())
    }

    println!("GET {}",format!("https://lrclib.net/api/get?artist_name={}&track_name={}",slugify!(artist_name,separator="+"),slugify!(song_name,separator="+")));
    return Ok(request.json::<Lrclib>().await.unwrap());
}








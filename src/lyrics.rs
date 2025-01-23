
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
    syncedLyrics: String
}

async fn get_lyrics_data(artist_name: &str, song_name: &str) -> Lrclib {
    let json: Result<Lrclib, reqwest::Error> = reqwest::get(
        format!("https://lrclib.net/api/get?artist_name={}&track_name={}",slugify!(artist_name,separator="+"),slugify!(song_name,separator="+"))
    )
        .await
        .unwrap()
        .json::<Lrclib>()
        .await;
    println!("GET {}",format!("https://lrclib.net/api/get?artist_name={}&track_name={}",slugify!(artist_name,separator="+"),slugify!(song_name,separator="+")));
    return json.unwrap();
}


pub async fn get_lyrics(artist_name: &str, song_name: &str) -> Lrclib {
    return get_lyrics_data(artist_name, song_name).await;
}





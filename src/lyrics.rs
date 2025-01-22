use scraper::Html;
use ureq;

pub fn get_lyrics(artist_name: &str, song_name: &str) -> str {
    ureq::get(format!("https://genius.com/{}-{}-lyrics",artist_name,song_name))
    .header("User-Agent","Mozilla/5.0 (Linux; Android 12; DE2118) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Mobile Safari/537.36");
}


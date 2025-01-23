
use slugify::slugify;
use scraper::Html;
use scraper::Selector;
use reqwest;


async fn get_lyrics_data(artist_name: &str, song_name: &str) -> String {
    let html: Result<String, reqwest::Error> = reqwest::get(
        format!("https://genius.com/{}-{}-lyrics",slugify!(artist_name),slugify!(song_name))
    )
        .await
        .unwrap()
        .text()
        .await;
    println!("Getting {}",format!("https://genius.com/{}-{}-lyrics",slugify!(artist_name),slugify!(song_name)));
    return html.unwrap();
}


fn html_to_discord_formatter(html: &mut String) {
    *html = html
        .replace("<i>","*")
        .replace("</i>","*")
        .replace("<b>","**")
        .replace("</b>","**")
        .replace("<br>","\n")
        .replace("&amp","&")
}

fn process_lyrics(html: &str) -> String {
    let document: Html = Html::parse_document(html);
    let selector = Selector::parse(r#"[data-lyrics-container="true"] > a > span, [data-lyrics-container="true"] > * "#).unwrap();

    let mut html:String = String::new();

    for lyrics_part in document.select(&selector) {
        html += &lyrics_part.text().collect::<Vec<_>>()[0];
        html += "\n"; // This was the easiest way I could think of 
    }
    html_to_discord_formatter(&mut html);
    return html;
}

pub async fn get_lyrics(artist_name: &str, song_name: &str) -> String {
    return process_lyrics(get_genius_page(artist_name, song_name).await.as_str());
}





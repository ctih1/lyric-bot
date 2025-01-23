use std::env;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize)]
pub struct TranslateResult {
    pub detected_source_language:String,
    pub text:String
}

#[derive(Deserialize)]
pub struct Translation {
    pub translations: Vec<TranslateResult>
}

#[derive(Serialize)]
pub struct TranslationPayload {
    text: [String;1],
    target_lang: String
}


pub async fn translate(input:&String) -> Translation {
    let api_key: String = env::var("DEEPL_KEY").expect("Expected a DEEPL Api key...");
    let pl: TranslationPayload = TranslationPayload {
        text: [String::from(input)],
        target_lang: String::from("EN")
    };


    let client: reqwest::Client = reqwest::Client::new();
    
    let response: reqwest::Response = client
        .post("https://api-free.deepl.com/v2/translate")
        .header("Authorization", "DeepL-Auth-Key ".to_owned() + &api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&pl).unwrap())
        .send()
        .await.unwrap();

    
    return response.json().await.unwrap();

}
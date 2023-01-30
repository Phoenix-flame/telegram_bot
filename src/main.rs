
use telebot::Bot;
use std::env;
use futures::stream::Stream;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use telebot::functions::*;
fn main() {
    // Create the bot
    let mut bot = Bot::new(&env::var("6147407906:AAEp1nNMP2Y5YaRIrNAsVVa0fnZsL7-eYV4").unwrap()).update_interval(200);
 
    // Register a reply command which answers a message
    let handle = bot.new_cmd("/reply")
        .and_then(|(bot, msg)| {
            let mut text = msg.text.unwrap().clone();
            if text.is_empty() {
                text = "<empty>".into();
            }
 
            bot.message(msg.chat.id, text).send()
        })
        .for_each(|_| Ok(()));
 
    bot.run_with(handle);
}

async fn get_latest_news() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get("https://news.ycombinator.com/")
        .send()
        .await?;
    let document = Document::from_read(response.bytes().await?.as_ref()).unwrap();
    let mut latest_news = String::new();
    // Find the latest news
    for node in document.find(Name("a")).take(30) {
        latest_news.push_str(&format!("{}\n", node.text()));
    }

    Ok(latest_news)
}
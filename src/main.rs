
use futures::Future;
use select::predicate::Predicate;
use teloxide::prelude::*;
use std::env;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Class;
use futures::stream::Stream;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
    // let mut bot = Bot::new("6147407906:AAEp1nNMP2Y5YaRIrNAsVVa0fnZsL7-eYV4").update_interval(200);

    // let handle = bot.new_cmd("/news")
    //     .and_then(|(bot, msg)| {
    //         println!("{:?}", msg);
    //         bot.message(msg.chat.id, msg.text).send()
    //     })
    //     .for_each(|_| Ok(()));


    //     // .and_then(|(bot, msg)| {
    //     //     let news = get_latest_news().await.unwrap();
 
    //     //     
    //     // })
    //     // .for_each(|_| Ok(()));
 
    // bot.run_with(handle);
    Ok(())
}

async fn get_latest_news() -> Result<String, reqwest::Error> {
    let http = reqwest::Proxy::http("127.0.0.1:20170").unwrap();
    let client = reqwest::Client::builder().proxy(http).build()?;
    
    let response = client.get("https://hckrnews.com/")
        .send()
        .await?;
    let document = Document::from_read(response.bytes().await?.as_ref()).unwrap();
    let mut latest_news = String::new();
    let nodes = document.find(Class("story").and(Class("link"))).take(30);
    let filtered_nodes = nodes.filter(|x| x.text() != "undefined \n");
    for node in filtered_nodes {
        latest_news.push_str(&format!("{}\n", node.text()));
    }

    Ok(latest_news)
}
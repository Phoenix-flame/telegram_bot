
use futures::Future;
use select::predicate::Predicate;
use teloxide::utils::command;
use teloxide::{prelude::*, utils::command::BotCommands};
use std::env;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Class;
use futures::stream::Stream;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command()]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get latest hacker news")]
    News,
    
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => bot.send_message(msg.chat.id, "Welcome to my rust-bot :)").await?,
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::News => {
            let news = get_latest_news().await.unwrap();
            bot.send_message(msg.chat.id, news).await?
        }
    };

    Ok(())
}

async fn get_latest_news() -> Result<String, reqwest::Error> {
    // let http = reqwest::Proxy::http("127.0.0.1:20170").unwrap();
    let client = reqwest::Client::new();
    
    let response = client.get("https://hckrnews.com/")
        .send()
        .await?;
    let document = Document::from_read(response.bytes().await?.as_ref()).unwrap();
    let mut latest_news = String::new();
    let nodes = document.find(Class("story").and(Class("link"))).take(30);
    let filtered_nodes = nodes.filter(|x| x.text() != "undefined \n");
    for node in filtered_nodes {
        latest_news.push_str(&format!("<a>{}</a>\n", node.text()));
    }

    Ok(latest_news)
}
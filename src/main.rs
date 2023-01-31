
use futures::Future;
use select::predicate::Predicate;
use teloxide::types::InputFile;
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
    // let news = get_latest_news().await.unwrap();
    // println!("{:?}", news);
    // let http = reqwest::Proxy::http("127.0.0.1:20170").unwrap();
    // let client = reqwest::Client::builder().proxy(http).build()?;
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
        Command::Start => {
            // bot.set_chat_menu_button(
            //     vec![
            //         Command::Start.into(),
            //         Command::Help.into(),
            //         Command::News.into(),
            //     ],
            //     teloxide::types::InlineKeyboardButtonType::Callback,
            // );
            let response = reqwest::get("https://hckrnews.com/img/touch/apple-touch-icon-114x114-precomposed.png").await?;
            let photo = response.bytes().await?;
            bot.set_chat_photo(msg.chat.id, InputFile::memory(photo))
                .await?;
            bot.send_message(msg.chat.id, "Welcome to my rust-bot :)").await?
        },
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::News => {
            let news = get_latest_news().await.unwrap();
            bot.send_message(msg.chat.id, news).parse_mode(teloxide::types::ParseMode::Html).await?
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
        latest_news.push_str(&format!("<a href=\"{}\">{}</a>\n", node.attr("href").unwrap(), node.text()));
    }

    Ok(latest_news)
}
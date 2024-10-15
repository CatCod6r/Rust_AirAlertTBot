use reqwest::header::HeaderValue;
use std::error::Error;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, utils::command::BotCommands};

mod api_connector;
mod event_loop;

#[tokio::main]
async fn main() {
    let api_value = get_api_tokens().await.unwrap();
    Command::repl(api_value.bot, answer).await;
}

struct ApiValue {
    bot: Bot,
    header_value: HeaderValue,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]

enum Command {
    #[command(description = "Starts the bot")]
    Start,
    #[command(description = "Stop the bot")]
    Stop,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let api_value = get_api_tokens().await.unwrap();
    let mut should_stop: bool;
    match cmd {
        Command::Start => {
            should_stop = false;
            bot.send_message(msg.chat.id, "Starting the bot").await?; //TODO Make it check for cfg
            tokio::spawn(async move {
                event_loop::run_alert_loop(&msg.chat.id, bot, api_value.header_value, &should_stop)
                    .await;
            });
        }
        Command::Stop => {
            bot.send_message(msg.chat.id, "Stops the bot").await?;
            should_stop = true;
        }
    };
    Ok(())
}
async fn get_api_tokens() -> Result<(ApiValue), Box<dyn Error>> {
    let _ = dotenv_vault::dotenv();
    let bot_token = std::env::var("TBOT_TOKEN").unwrap_or("".to_string());
    let api_token = std::env::var("API_TOKEN").unwrap_or("".to_string());
    let api_value = ApiValue {
        bot: Bot::new(bot_token),
        header_value: HeaderValue::from_str(&api_token)?,
    };

    Ok(api_value)
}

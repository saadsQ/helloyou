use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::dispatching::UpdateFilterExt;

#[derive(BotCommands)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "start the bot.")]
    Start,
}

async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Hello! The bot is now running.").await?;
        }
    }
    Ok(())
}

pub async fn run() {
    let bot = Bot::from_env();

    Dispatcher::new(bot.clone())
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.commands::<Command, &str>(Command::ty(), move |bot, msg, cmd| {
                handle_command(bot, msg, cmd)
            })
        })
        .dispatch()
        .await;
}
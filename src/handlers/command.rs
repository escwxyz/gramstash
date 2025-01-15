use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::dispatching::{HandlerExt, UpdateHandler};
use teloxide::prelude::*;
use teloxide::{types::Message, Bot};

use crate::command::Command;
use crate::error::{BotError, HandlerResult};
use crate::services::dialogue::DialogueState;
use crate::state::AppState;
use crate::utils::keyboard::{self};

use super::RequestContext;

async fn handle_language(bot: Bot, msg: Message) -> HandlerResult<()> {
    bot.send_message(msg.chat.id, t!("commands.language"))
        .reply_markup(keyboard::LanguageMenu::get_language_menu_inline_keyboard())
        .await?;

    Ok(())
}

async fn handle_start(
    bot: Bot,
    state: &AppState,
    dialogue: Dialogue<DialogueState, ErasedStorage<DialogueState>>,
    msg: Message,
    ctx: RequestContext,
) -> HandlerResult<()> {
    let RequestContext {
        telegram_user_id,
        telegram_user_name,
        ..
    } = ctx;

    let auth_service = state.auth.lock().await;

    let is_authenticated = auth_service.is_authenticated(&telegram_user_id.to_string()).await?;

    let welcome_text = if is_authenticated {
        t!(
            "commands.start.authenticated",
            first_name = telegram_user_name,
            telegram_user_id = telegram_user_id.to_string()
        )
    } else {
        t!(
            "commands.start.unauthenticated",
            first_name = telegram_user_name,
            telegram_user_id = telegram_user_id.to_string()
        )
    };

    bot.send_message(msg.chat.id, welcome_text)
        .reply_markup(keyboard::MainMenu::get_inline_keyboard())
        .await?;

    dialogue
        .update(DialogueState::Start)
        .await
        .map_err(|e| BotError::DialogueStateError(e.to_string()))?;

    Ok(())
}

async fn handle_help(bot: Bot, msg: Message) -> HandlerResult<()> {
    bot.send_message(msg.chat.id, t!("commands.help"))
        .reply_markup(keyboard::MainMenu::get_inline_keyboard())
        .await?;

    Ok(())
}

async fn handle_unknown_command(bot: Bot, msg: Message) -> HandlerResult<()> {
    bot.send_message(msg.chat.id, t!("commands.unknown_command")).await?;
    Ok(())
}

async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: Dialogue<DialogueState, ErasedStorage<DialogueState>>,
    state: &AppState,
    ctx: RequestContext,
) -> HandlerResult<()> {
    match cmd {
        Command::Start => handle_start(bot, state, dialogue, msg, ctx).await?,
        Command::Help => handle_help(bot, msg).await?,
        Command::Language => handle_language(bot, msg).await?,
        Command::Stats | Command::Status if !ctx.is_admin => handle_unknown_command(bot, msg).await?,
        _ => handle_unknown_command(bot, msg).await?,
    }

    Ok(())
}

pub fn get_command_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync>> {
    Update::filter_message()
        .filter_command::<Command>()
        .endpoint(handle_command)
}

#[cfg(test)]
mod tests {
    use crate::{services::dialogue::DialogueState, utils::test::setup_test_bot};

    #[tokio::test]
    async fn test_handle_help() {
        let bot = setup_test_bot("/help").await;
        bot.set_state(DialogueState::Start).await;

        bot.dispatch().await;

        let responses = bot.get_responses();
        let last_message = responses.sent_messages.last().expect("No messages were sent");

        assert_eq!(last_message.text().expect("Message had no text"), t!("commands.help"));

        assert!(
            last_message.reply_markup().is_some(),
            "Expected reply markup to be present"
        );
    }

    #[tokio::test]
    async fn test_handle_unknown_command() {
        let bot = setup_test_bot("/stats").await;

        bot.dispatch().await;

        let responses = bot.get_responses();
        let last_message = responses.sent_messages.last().expect("No messages were sent");

        assert_eq!(
            last_message.text().expect("Message had no text"),
            t!("commands.unknown_command")
        );
    }
}

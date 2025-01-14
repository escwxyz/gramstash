use teloxide::{
    dispatching::dialogue::ErasedStorage,
    payloads::EditMessageTextSetters,
    prelude::{Dialogue, Requester},
    types::{Message, MessageId},
    Bot,
};

use crate::{
    error::{BotError, HandlerResult, MiddlewareError, ServiceError},
    services::{
        auth::{reconstruct_raw_text, Credentials},
        dialogue::DialogueState,
        middleware::process_instagram_username,
    },
    state::AppState,
    utils::{keyboard, validate_instagram_password},
};

pub(super) async fn handle_message_username(
    bot: Bot,
    dialogue: Dialogue<DialogueState, ErasedStorage<DialogueState>>,
    msg: Message,
    prompt_msg_id: MessageId,
) -> HandlerResult<()> {
    info!("handle_message_username");

    bot.delete_message(msg.chat.id, prompt_msg_id).await?;

    let entities = msg.parse_entities();

    let text = msg.text().ok_or_else(|| {
        BotError::ServiceError(ServiceError::Middleware(MiddlewareError::Other(
            "No username provided".into(),
        )))
    })?;

    let raw_text = if let Some(entities) = entities {
        reconstruct_raw_text(text, &entities)
    } else {
        text.to_string()
    };

    info!("raw_text: {:?}", raw_text);

    let username_input = msg.text().ok_or_else(|| {
        BotError::ServiceError(ServiceError::Middleware(MiddlewareError::Other(
            "No username provided".into(),
        )))
    })?;

    info!("username_input: {:?}", username_input);

    let validating_msg = bot
        .send_message(msg.chat.id, t!("messages.profile.username.validating"))
        .await?;

    let username = match process_instagram_username(&raw_text) {
        Ok(username) => username,
        Err(_) => {
            bot.edit_message_text(
                msg.chat.id,
                validating_msg.id,
                t!(
                    "messages.profile.username.invalid",
                    username = username_input.to_string()
                ),
            )
            .await?;
            dialogue
                .update(DialogueState::AwaitingUsername(msg.id))
                .await
                .map_err(|e| BotError::DialogueStateError(e.to_string()))?;
            return Ok(());
        }
    };

    // TODO

    // let session_service = AppState::get()?.session.lock().await;

    // let telegram_user_id = msg.clone().from.unwrap().id.to_string();

    // // Check if there's a valid session for this user
    let session_msg = bot
        .edit_message_text(
            msg.chat.id,
            validating_msg.id,
            t!("messages.profile.username.validating_session"),
        )
        .await?;

    // if session_service.validate_session(&telegram_user_id).await? {
    //     // If session exists and is valid, check if it matches the provided username
    //     if let Some(stored_session) = session_service.get_session(&telegram_user_id).await? {
    //         if let Some(session_data) = &stored_session.session_data {
    //             if session_data.username == Some(username.clone()) {
    //                 bot.edit_message_text(
    //                     msg.chat.id,
    //                     session_msg.id,
    //                     t!("messages.profile.username.validating_session_success"),
    //                 )
    //                 .reply_markup(keyboard::MainMenu::get_inline_keyboard())
    //                 .await?;
    //                 return Ok(());
    //             }
    //         }
    //     }
    // }

    let password_msg = bot
        .edit_message_text(
            msg.chat.id,
            session_msg.id,
            t!(
                "messages.profile.username.invalid_session",
                username = username.to_string()
            ),
        )
        .reply_markup(keyboard::LoginDialogue::get_cancel_auth_keyboard())
        .await?;

    dialogue
        .update(DialogueState::AwaitingPassword {
            username: username.to_string(),
            prompt_msg_id: password_msg.id,
        })
        .await?;
    // TODO: test this
    bot.delete_message(msg.chat.id, msg.id).await?;

    Ok(())
}

pub(super) async fn handle_message_password(
    bot: Bot,
    dialogue: Dialogue<DialogueState, ErasedStorage<DialogueState>>,
    msg: Message,
    (username, prompt_msg_id): (String, MessageId),
) -> HandlerResult<()> {
    bot.delete_message(msg.chat.id, prompt_msg_id).await?;

    let text = msg.text().ok_or_else(|| {
        BotError::ServiceError(ServiceError::Middleware(MiddlewareError::Other(
            "No password provided".into(),
        )))
    })?;

    let entities = msg.parse_entities();

    let raw_text = if let Some(entities) = entities {
        reconstruct_raw_text(text, &entities)
    } else {
        text.to_string()
    };

    info!("raw_text: {:?}", raw_text);

    if !validate_instagram_password(&raw_text) {
        bot.delete_message(msg.chat.id, msg.id).await?;
        bot.send_message(msg.chat.id, t!("messages.profile.password.invalid"))
            .await?;

        dialogue
            .update(DialogueState::AwaitingPassword {
                username,
                prompt_msg_id: msg.id,
            })
            .await?;
        return Ok(());
    }

    // IMPORTANT: Delete the password message immediately
    bot.delete_message(msg.chat.id, msg.id).await?;

    let status_msg = bot
        .send_message(msg.chat.id, t!("messages.profile.password.logging_in"))
        .await?;

    let state = AppState::get()?;

    let mut auth_service = state.auth.lock().await;

    match auth_service
        .login(Credentials {
            username,
            password: raw_text,
        })
        .await
    {
        Ok(_session_data) => {
            let _telegram_user_id = msg.from.unwrap().id.to_string();
            // TODO
            // session_service
            //     .sync_session(&telegram_user_id, session_data, true)
            //     .await?;

            bot.edit_message_text(
                msg.chat.id,
                status_msg.id,
                t!("messages.profile.password.login_success"),
            )
            .reply_markup(keyboard::MainMenu::get_inline_keyboard())
            .await?;
        }
        Err(e) => {
            let msg = bot
                .edit_message_text(
                    msg.chat.id,
                    status_msg.id,
                    t!("messages.profile.password.login_failed", error = e.to_string()),
                )
                .await?;

            dialogue
                .update(DialogueState::AwaitingUsername(msg.id))
                .await
                .map_err(|e| BotError::DialogueStateError(e.to_string()))?;

            return Ok(());
        }
    }

    Ok(())
}

use crate::{cfg, BotErr};
use llm::InferenceResponse;
use teloxide::prelude::*;

pub async fn is_subscribed_to_chan(
    bot: &Bot,
    cfg: &cfg::Config,
    msg: &Message,
) -> Result<bool, BotErr> {
    let chat_id = msg.chat.id;

    let Some(user) = msg.from() else {
        bot.send_message(chat_id, "Не могу получить информацию о тебе").await?;
        return Err(BotErr::from(""));
    };

    Ok(bot
        .get_chat_member(cfg.channel_id, user.id)
        .await?
        .is_present())
}

pub async fn update_message(
    bot: &Bot,
    new_token: InferenceResponse,
    msg: &Message,
) -> Result<(), BotErr> {
    let chat_id = msg.chat.id;

    bot.edit_message_text(chat_id, msg.id, "").await?;

    Ok(())
}

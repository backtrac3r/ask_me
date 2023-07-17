mod cfg;
mod helpers;
mod llm;

use helpers::is_subscribed_to_chan;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type BotErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let bot = Bot::from_env();

    let cfg = Arc::new(cfg::Config::init());

    let models_cfg = Arc::new(llm::Config::init());

    Dispatcher::builder(
        bot,
        dptree::entry().branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(dptree::case![State::Start].endpoint(start)),
        ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new(), cfg, models_cfg])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

#[allow(clippy::missing_errors_doc)]
pub async fn start(
    bot: Bot,
    _dialogue: MyDialogue,
    models_cfg: Arc<llm::Config>,
    cfg: Arc<cfg::Config>,
    msg: Message,
) -> Result<(), BotErr> {
    let chat_id = msg.chat.id;

    let Some(txt) = msg.text() else {
        bot.send_message(chat_id, "Нужно отправить текст").await?;
        return Ok(());
    };

    if txt == "/start" {
        bot.send_message(chat_id, "Задай мне вопрос").await?;
        return Ok(());
    }

    if !is_subscribed_to_chan(&bot, &cfg, &msg).await? {
        bot.send_message(
            chat_id,
            format!(
                "Что бы использовать бота, подпишись на канал -> @{}",
                cfg.channel_name
            ),
        )
        .await?;
        return Ok(());
    }

    models_cfg.get_ans(&bot, &msg);

    Ok(())
}

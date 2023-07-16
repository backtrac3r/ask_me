use std::env;
use teloxide::types::ChatId;

#[derive(Clone)]
pub struct Config {
    pub creator_id: ChatId,
    pub creator_pass: String,
    pub bot_name: String,
    pub channel_id: ChatId,
    pub channel_name: String,
}

impl Config {
    pub fn init() -> Self {
        let creator_id = ChatId(env::var("CREATOR_ID").unwrap().parse().unwrap());
        let creator_pass = env::var("CREATOR_PASS").unwrap();
        let bot_name = env::var("BOT_NAME").unwrap().parse().unwrap();
        let channel_id = ChatId(env::var("CHANNEL_ID").unwrap().parse().unwrap());
        let channel_name = env::var("CHANNEL_NAME").unwrap();

        Self {
            creator_id,
            creator_pass,
            bot_name,
            channel_id,
            channel_name,
        }
    }
}

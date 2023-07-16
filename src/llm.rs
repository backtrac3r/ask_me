use llm::{InferenceSessionConfig, Model, Prompt, InferenceResponse};
use tokio::sync::Mutex;
use std::{path::PathBuf, sync::{Arc, mpsc::{SyncSender, sync_channel, Receiver}}};
use teloxide::prelude::*;

use crate::helpers::update_message;

pub struct Config {
    llm_model: Box<dyn Model>,
}

impl Config {
    pub async fn get_ans(&self, bot: &Bot, msg: &Message) {
        let session = self
            .llm_model
            .start_session(InferenceSessionConfig::default());

        let chat_id = msg.chat.id;
        let txt = msg.text().unwrap_or_default();

        session.infer::<std::convert::Infallible>(
            self.llm_model.as_ref(),
            &mut rand::thread_rng(),
            &llm::InferenceRequest {
                prompt: Prompt::Text(txt),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            |r| {
                update_message(, , );
                Ok(llm::InferenceFeedback::Continue)
            },
        );
    }

    pub fn init() -> Self {
        let vocabulary_source = llm::TokenizerSource::Embedded;
        let model_path = PathBuf::from("./open_llama_7b-q4_0-ggjt.bin");

        let now = std::time::Instant::now();

        let llm_model = llm::load_dynamic(
            Some(llm::ModelArchitecture::Llama),
            &model_path,
            vocabulary_source,
            Default::default(),
            llm::load_progress_callback_stdout,
        )
        .unwrap();

        println!(
            "Model fully loaded! Elapsed: {}ms",
            now.elapsed().as_millis()
        );

        let (sender, receiver) = sync_channel(1);

        let tx_infer: Arc<Mutex<SyncSender<InferenceResponse>>> = Arc::new(Mutex::new(sender));
        let rx_callback: Arc<Mutex<Receiver<InferenceResponse>>> = Arc::new(Mutex::new(receiver));

        Self { llm_model }
    }
}

use llm::{
    load_dynamic, load_progress_callback_stdout, InferenceResponse, InferenceSessionConfig, Model,
    ModelArchitecture, ModelParameters, Prompt,
};
use std::{
    path::PathBuf,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};
use teloxide::prelude::*;
use tokio::sync::Mutex;

use crate::helpers::update_message;

pub struct Config {
    llm_model: Box<dyn Model>,
}

impl Config {
    pub fn get_ans(&self, bot: &Bot, msg: &Message) {
        let mut session = self
            .llm_model
            .start_session(InferenceSessionConfig::default());

        let chat_id = msg.chat.id;
        let txt = msg.text().unwrap_or_default();

        let res = session.infer::<std::convert::Infallible>(
            self.llm_model.as_ref(),
            &mut rand::thread_rng(),
            &llm::InferenceRequest {
                prompt: Prompt::Text(txt),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            |t| {
                match t {
                    InferenceResponse::SnapshotToken(t) => print!("{t}"),
                    InferenceResponse::PromptToken(t) => print!("{t}"),
                    InferenceResponse::InferredToken(t) => print!("{t}"),
                    InferenceResponse::EotToken => todo!(),
                }

                Ok(llm::InferenceFeedback::Continue)
            },
        );

        match res {
            Ok(result) => println!("\n\nInference stats:\n{result}"),
            Err(err) => println!("\n{err}"),
        }
    }

    pub fn init() -> Self {
        let vocabulary_source = llm::TokenizerSource::Embedded;
        let model_path = PathBuf::from("./open_llama_7b-q4_0-ggjt.bin");

        let now = std::time::Instant::now();

        let llm_model = load_dynamic(
            Some(ModelArchitecture::Llama),
            &model_path,
            vocabulary_source,
            ModelParameters::default(),
            load_progress_callback_stdout,
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

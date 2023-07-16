use rust_bert::pipelines::{
    common::ModelType,
    conversation::{ConversationConfig, ConversationManager, ConversationModel},
    translation::{Language, TranslationModel, TranslationModelBuilder},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Config {
    conversation_model: Arc<Mutex<ConversationModel>>,
    conversation_manager: Arc<Mutex<ConversationManager>>,
    ru_en: Arc<Mutex<TranslationModel>>,
    en_ru: Arc<Mutex<TranslationModel>>,
}

impl Config {
    pub fn init() -> Self {
        let conversation_model = Arc::new(Mutex::new(
            ConversationModel::new(ConversationConfig::default()).unwrap(),
        ));
        let conversation_manager = Arc::new(Mutex::new(ConversationManager::new()));
        let ru_en = Arc::new(Mutex::new(
            TranslationModelBuilder::new()
                .with_model_type(ModelType::Marian)
                .with_source_languages(vec![Language::Russian])
                .with_target_languages(vec![Language::English])
                .create_model()
                .unwrap(),
        ));
        let en_ru = Arc::new(Mutex::new(
            TranslationModelBuilder::new()
                .with_model_type(ModelType::Marian)
                .with_source_languages(vec![Language::English])
                .with_target_languages(vec![Language::Russian])
                .create_model()
                .unwrap(),
        ));

        Self {
            conversation_model,
            conversation_manager,
            ru_en,
            en_ru,
        }
    }

    pub async fn ru_to_en(&self, ru_txt: &str) -> String {
        self.ru_en
            .lock()
            .await
            .translate(&[ru_txt], Language::Russian, Language::English)
            .unwrap()[0]
            .clone()
    }

    pub async fn en_to_ru(&self, en_txt: &str) -> String {
        self.en_ru
            .lock()
            .await
            .translate(&[en_txt], Language::English, Language::Russian)
            .unwrap()[0]
            .clone()
    }

    pub async fn get_ans(&self, ru_q: &str) -> String {
        let en_q = self.ru_to_en(ru_q).await;

        let mut conversation_manager = self.conversation_manager.lock().await;
        let q_id = conversation_manager.create(&en_q);
        let output = self
            .conversation_model
            .lock()
            .await
            .generate_responses(&mut conversation_manager);

        self.en_to_ru(output.get(&q_id).unwrap()).await
    }
}

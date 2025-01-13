use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    config::AppConfig,
    error::BotResult,
    services::dialogue::{DialogueService, DialogueState},
    state::AppState,
};
use teloxide::dispatching::dialogue::ErasedStorage;

#[allow(unused)]
pub static TEST_MUTEX: Mutex<()> = Mutex::const_new(());
#[allow(unused)]
fn get_redis_host() -> String {
    std::env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
}
#[allow(unused)]
/// Common test setup function that can be used across all test files
pub async fn setup_test_state() -> BotResult<(&'static AppState, Arc<ErasedStorage<DialogueState>>)> {
    let _lock = TEST_MUTEX.lock().await;

    let mut test_config = AppConfig::new_test_config();

    let redis_host = get_redis_host();
    test_config.redis.url = format!("redis://{}:6379", redis_host);
    test_config.dialogue.redis_url = format!("redis://{}:6379", redis_host);

    // Only initialize if not already initialized
    if AppState::get().is_err() {
        AppState::init_test_with_config(test_config.clone())
            .await
            .expect("Failed to initialize test app state");
    }

    let test_app_state = AppState::get()?;

    let storage = DialogueService::get_dialogue_storage(&test_config.dialogue)
        .await
        .expect("Failed to initialize test storage");

    Ok((test_app_state, storage))
}
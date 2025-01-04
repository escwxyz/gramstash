use anyhow::Context;
use redis::AsyncCommands;
use teloxide::types::ChatId;

use crate::{state::AppState, utils::error::BotResult};

pub struct RateLimiter {
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        let state = AppState::get();
        Self {
            max_requests: state.config.rate_limit.daily_limit,
            window_seconds: state.config.rate_limit.window_secs,
        }
    }

    pub async fn check_rate_limit(&self, chat_id: ChatId) -> BotResult<bool> {
        let mut conn = AppState::get().redis.get_connection().await?;
        let key = format!("rate_limit:{}:{}", chat_id.0, chrono::Utc::now().date_naive());

        let exists: bool = conn.exists(&key).await.context("Failed to check key existence")?;

        if !exists {
            conn.set_ex::<_, _, String>(&key, 1, self.window_seconds)
                .await
                .context("Failed to set initial rate limit")?;
            return Ok(true);
        }

        let counter: u32 = conn.incr(&key, 1).await.context("Failed to increment counter")?;

        Ok(counter <= self.max_requests)
    }
}

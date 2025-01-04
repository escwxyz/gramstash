use crate::{
    services::instagram::MediaInfo,
    state::AppState,
    utils::error::{BotError, BotResult},
};

use redis::AsyncCommands;
use serde_json;

pub struct CacheService;

impl CacheService {
    pub async fn get_media_info(shortcode: &str) -> BotResult<Option<MediaInfo>> {
        let mut conn = AppState::get().redis.get_connection().await?;
        let key = Self::generate_key(shortcode);

        let data: Option<String> = conn.get(&key).await.map_err(|e| BotError::CacheError(e.to_string()))?;

        match data {
            Some(json) => Ok(Some(
                serde_json::from_str(&json).map_err(|e| BotError::CacheError(e.to_string()))?,
            )),
            None => Ok(None),
        }
    }

    pub async fn set_media_info(shortcode: &str, media_info: &MediaInfo) -> BotResult<()> {
        let expiry_secs = AppState::get().config.cache.expiry_secs;
        let mut conn = AppState::get().redis.get_connection().await?;
        let key = Self::generate_key(shortcode);

        let json = serde_json::to_string(media_info).map_err(|e| BotError::CacheError(e.to_string()))?;
        conn.set_ex::<_, _, String>(&key, json, expiry_secs)
            .await
            .map_err(|e| BotError::CacheError(e.to_string()))?;

        Ok(())
    }

    fn generate_key(shortcode: &str) -> String {
        format!("media_cache:{}", shortcode)
    }
}

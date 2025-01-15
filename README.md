# GramStash (An Instagram Media Downloader) 🤖

[![CI](https://github.com/escwxyz/gramstash/actions/workflows/test.yml/badge.svg)](https://github.com/escwxyz/gramstash/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/escwxyz/gramstash/graph/badge.svg?token=UEAL5KGTVW)](https://codecov.io/gh/escwxyz/gramstash)

A Telegram bot that allows users to download media content from Instagram, including posts, reels, and stories. Built with Rust for performance.

## Features 🌟

- Download Instagram posts and reels
- Rate limiting & Cache, powered by Upstash
- Access and download Instagram stories (Coming soon)
- Private chat only (no group chat support)
- Secure authentication handling (Coming soon)
- Redis support for session storage, powered by Upstash
- Shuttle for easy deployment

## Getting Started 🚀

### Prerequisites

- Rust 1.70 or higher
- A Telegram Bot Token (get it from [@BotFather](https://t.me/botfather))
- [Upstash](https://upstash.com/) account (for Redis and Rate Limiting)
- [Shuttle](https://shuttle.dev/) account (for deployment)
- [Turso](https://turso.tech/) account (for database)

### Environment Variables

Create a `Secret.toml` file in the root directory:

```toml
UPSTASH_REDIS_HOST = "your-redis-host"
UPSTASH_REDIS_PASSWORD = "your-redis-password"
UPSTASH_REDIS_PORT = "your-redis-port"
TELEGRAM_BOT_TOKEN = "your-telegram-bot-token"
INSTAGRAM_API_ENDPOINT = "https://www.instagram.com/graphql/query/"
INSTAGRAM_DOC_ID = "your-instagram-doc-id"
RATE_LIMIT_DAILY_LIMIT = "your-rate-limit-daily-limit"
RATE_LIMIT_WINDOW_SECS = "your-rate-limit-window-secs"
CACHE_EXPIRY_SECS = "your-cache-expiry-secs"
DIALOGUE_USE_REDIS = "true to use redis, false to use in-memory"
DIALOGUE_CLEAR_INTERVAL_SECS = "your-dialogue-clear-interval-secs"
SESSION_REFRESH_INTERVAL_SECS = "your-session-refresh-interval-secs"
TURSO_URL = "your-turso-database-url"
TURSO_TOKEN = "your-turso-token"
```

## Architecture 🏗️

- `src/bot.rs` - Core bot logic and dialogue state management
- `src/config.rs` - Configuration management
- `src/handlers/` - Message and callback handlers
- `src/services/` - Instagram API integration and related services
- `src/utils/` - Helper functions and error handling

## Roadmap 🛣️

- [x] Basic post/reel downloading
- [x] Redis integration
- [x] Rate limiting
- [x] Dialogue state management
- [x] Unit tests
- [ ] Session management
- [ ] Story downloading
- [ ] Highlight downloading
- [x] Internationalization
- [ ] Metrics
- [ ] Monetization

...

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust coding conventions
- Add tests for new features
- Update documentation as needed
- Keep commits clean and well-documented

## Known Issues 🐛

- Due to the current limitation of `teloxide_tests`, we have to use the default `Bot` in `BotService` without any adapters. See [here](https://github.com/LasterAlex/teloxide_tests/issues/25) for more details.
- User input is automatically processed by Telegram, currently we restore the display text back to the raw text, which is most likely not the perfect solution.

## Security 🔒

- User credentials are never stored permanently
- Messages containing sensitive information are automatically deleted
- Session tokens are stored securely
- Group chat access is blocked

## License 📝

This project is licensed under the Apache 2.0 License.

See the [LICENSE](LICENSE) file for the complete license text.

## Acknowledgments 👏

- [Teloxide](https://github.com/teloxide/teloxide) - Telegram Bot Framework

## Support 💬

For support, please open an issue.

## Disclaimer ⚠️

This bot is not affiliated with Instagram or Meta. The codebase is only for educational purposes. Use it responsibly and in accordance with Instagram's terms of service. You are responsible for your own actions.

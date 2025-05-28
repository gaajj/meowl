# Meowl Bot

A Discord bot built with [Poise](https://docs.rs/poise), using slash and prefix commands.

## Features

For now there just basic commands

- `/help` – Overview of the bot
- `/commands` – Lists all available commands

## Setup

1. Copy `.env.example` to `.env` and fill in:

```env
DISCORD_TOKEN=your-bot-token-here
GUILD_ID=your-test-guild-id
```

1. Run the bot:

```bash
cargo run
```

Made using [Rust](https://www.rust-lang.org/) + [Serenity](https://docs.rs/serenity) + [Poise](https://docs.rs/poise)

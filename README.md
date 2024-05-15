# discord-translate-bot
![GitHub top language](https://img.shields.io/github/languages/top/CodeDead/discord-translate-bot)
![GitHub](https://img.shields.io/github/license/CodeDead/discord-translate-bot)
[![Release](https://github.com/CodeDead/discord-translate-bot/actions/workflows/release.yml/badge.svg)](https://github.com/CodeDead/discord-translate-bot/actions/workflows/release.yml)
[![Test](https://github.com/CodeDead/discord-translate-bot/actions/workflows/test.yml/badge.svg)](https://github.com/CodeDead/discord-translate-bot/actions/workflows/test.yml)

A simple discord bot that translates messages using the Google Cloud Translation API.

## Commands / Interactions

The bot has the following commands:

* `!translate [target-language]` - Reply to any message to translate it to the target language. If the target language is not specified, the bot will translate the message to English.
* `/translate-text [message] [target-language]` - Translate the provided message to the target language. If the target language is not specified, the bot will translate the message to English.
* `!help` - Display a list of available commands and useful information.
* `!register` - Register the `translate-text` command. This command is only available to users with the `ADMINISTRATOR` permission. 
* `!unregister` - Unregister the `translate-text` command. This command is only available to users with the `ADMINISTRATOR` permission.

## Installation

In order to run this bot, you need to have a [Google Cloud](https://cloud.google.com/translate) account and a project with the Cloud Translation API enabled. You also need to have a Discord bot token. You can get one by creating a new bot in the [Discord Developer Portal](https://discord.com/developers/docs/intro).

A `.env` file is required to run the bot. The file should contain the following variables:

```env
DISCORD_TOKEN=your_discord_bot_token
GOOGLE_TRANSLATE_TOKEN=your_google_cloud_translation_api_token
# Optionally, you can change the log level
RUST_LOG=info # or debug, trace, etc.
```

You can install and run the bot by cloning the repository and running the following command:

```bash
cargo run
```

To build a release version, run the following command:

```bash
cargo build --release
```

You can then run the release version using the following command:

```bash
./target/release/discord_translate_bot
```

## Dependencies

* [Rust](https://www.rust-lang.org/)
* [dotenvy](https://crates.io/crates/dotenvy)
* [env_logger](https://crates.io/crates/env_logger)
* [reqwest](https://crates.io/crates/reqwest)
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [serenity](https://crates.io/crates/serenity)
* [tokio](https://crates.io/crates/tokio)
* [log](https://crates.io/crates/log)

## About

This library is maintained by CodeDead. You can find more about us using the following links:
* [Website](https://codedead.com/)
* [Twitter](https://twitter.com/C0DEDEAD/)
* [Facebook](https://facebook.com/deadlinecodedead/)

Copyright © 2024 CodeDead

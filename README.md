# 🏎️ discord-bot-f1 🏁

Welcome to the world of Formula 1 through Discord! Our bot brings the excitement of F1 racing to your Discord server, offering a range of interactive commands that let users dive into the rich history and thrilling present of F1 racing.

## 🚦 Features & Commands
Our bot is equipped with several commands, each designed to bring a unique aspect of F1 to your fingertips:

### 1. 🗨️ `!quote`
   - Get a random, inspiring quote from the world of F1.

### 2. 🏎️ `!driver [number]`
   - Provide a number, and the bot fetches an F1 driver associated with that number, complete with an image.

### 3. 🏆 `!champion [year/name]`
   - Enter a year to know who clinched the Drivers' Championship, or input a driver's name to find out their championship years.

### 4. 🏁 `!constructor [year/team]`
   - Similar to `!champion`, this command provides information about Constructors' Championship winners by year or team.

### 5. 📊 `!points`
   - Engage in a fun F1 quiz! Answer questions, score points, and see who tops the leaderboard with their F1 knowledge.

### 6. 🎖️ `!leaderboard`
   - Reach 5 points in the quiz to become the F1 Quiz Champion! See the frequency of wins for each participant on the leaderboard.

## 📚 Documentation & Technical Details

To bring this bot to life, we've utilized several key Rust libraries:

- [**serenity**](https://crates.io/crates/serenity): A comprehensive Rust library for interacting with the Discord API.
- [**tokio**](https://crates.io/crates/tokio): Powers asynchronous operations crucial for a responsive Discord bot.
- [**rand**](https://crates.io/crates/rand): Adds the element of surprise by selecting quotes and quiz questions randomly.
- [**csv**](https://crates.io/crates/csv): Parses CSV files efficiently, turning raw data into usable information.
- [**serde**](https://crates.io/crates/serde): A serialization framework that seamlessly deserializes data into structs.

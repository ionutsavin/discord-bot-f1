use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, sync::Arc};
mod commands;
use commands::data_loading::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received message: {}", msg.content);

        if msg.content.starts_with("!quote") {
            self.process_quote(&ctx, &msg).await;
        } else if msg.content.starts_with("!driver") {
            self.process_driver(&ctx, &msg).await;
        } else if msg.content.starts_with("!champion") {
            self.process_champion(&ctx, &msg).await;
        } else if msg.content.starts_with("!constructor") {
            self.process_constructor(&ctx, &msg).await;
        } else if msg.content == "!points" {
            self.process_points_command(&ctx, &msg).await;
        } else if msg.content == "!leaderboard" {
            self.process_leaderboard_command(&ctx, &msg).await;
        }

        self.process_answer(&ctx, &msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Clone the data needed for asking questions
        let current_question = Arc::clone(&self.current_question);
        let question_answered = Arc::clone(&self.question_answered);
        let questions = self.questions.clone();

        tokio::spawn(async move {
            Handler::ask_question(ctx, current_question, question_answered, questions).await;
        });
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let handler = match Handler::new().await {
        Ok(handler) => handler,
        Err(why) => {
            println!("Handler error: {:?}", why);
            return;
        }
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

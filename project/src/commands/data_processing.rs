use super::data_loading::Handler;
use serenity::{
    all::{CreateAttachment, CreateMessage},
    model::{channel::Message, id::UserId},
    prelude::*,
};

impl Handler {
    // Processing the !quote command
    pub async fn process_quote(&self, ctx: &Context, msg: &Message) {
        if let Some(quote) = self.get_random_quote() {
            if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Processing the !driver command
    pub async fn process_driver(&self, ctx: &Context, msg: &Message) {
        let args: Vec<&str> = msg.content.split_whitespace().collect();
        match args.get(1) {
            Some(num_str)
                if num_str
                    .parse::<usize>()
                    .map_or(false, |num| (1..=99).contains(&num)) =>
            {
                let num = num_str.parse::<usize>().unwrap(); // Safe to unwrap because it's a valid number
                let file_path = format!("drivers_photos/{}-driver.jpg", num);

                // Check if the file exists
                if std::path::Path::new(&file_path).exists() {
                    let response = format!("This is Driver number {}", num);
                    let builder = CreateMessage::new()
                        .content(&response)
                        .add_file(CreateAttachment::path(&file_path).await.unwrap());
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                        println!("Error sending message: {:?}", why);
                    }
                } else {
                    let response = "No driver had this number.";
                    let _ = msg.channel_id.say(&ctx.http, response).await;
                }
            }
            _ => {
                // Default case for invalid input
                let response = "Please provide a valid driver number (1-99).";
                let _ = msg.channel_id.say(&ctx.http, response).await;
            }
        }
    }

    // Processing the !champion command
    pub async fn process_champion(&self, ctx: &Context, msg: &Message) {
        let args: Vec<&str> = msg.content.split_whitespace().collect();

        let response = if let Some(arg) = args.get(1) {
            match arg.parse::<usize>() {
                Ok(year) if (1950..=2023).contains(&year) => {
                    // Query by year
                    let year_str = arg.to_string();
                    self.champions
                        .iter()
                        .find(|champion| champion.year == year_str)
                        .map_or_else(
                            || "Drivers' Champion not found for the specified year.".to_string(),
                            |champion| {
                                format!(
                                    "Year: {}, Drivers' Champion: {}",
                                    champion.year, champion.driver
                                )
                            },
                        )
                }
                _ => {
                    // Query by driver name
                    let driver_name_query = args[1..].join(" ").to_lowercase();
                    let driver_name_formatted = driver_name_query
                        .split_whitespace()
                        .map(|name_part| {
                            name_part
                                .char_indices()
                                .map(|(i, c)| {
                                    if i == 0 {
                                        c.to_uppercase().to_string()
                                    } else {
                                        c.to_string()
                                    }
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    let years_list: Vec<String> = self
                        .champions
                        .iter()
                        .filter(|champion| champion.driver.to_lowercase() == driver_name_query)
                        .map(|champion| champion.year.clone())
                        .collect();

                    if years_list.is_empty() {
                        format!(
                            "No Drivers' Championship found for '{}'.",
                            driver_name_formatted
                        )
                    } else {
                        format!(
                            "{} won the Drivers' Championship in: {}",
                            driver_name_formatted,
                            years_list.join(", ")
                        )
                    }
                }
            }
        } else {
            "Please provide a year or a full driver name.".to_string()
        };

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }

    // Processing the !constructor command
    pub async fn process_constructor(&self, ctx: &Context, msg: &Message) {
        let args: Vec<&str> = msg.content.split_whitespace().collect();

        let response = if let Some(arg) = args.get(1) {
            match arg.parse::<usize>() {
                Ok(year) if (1958..=2023).contains(&year) => {
                    // Query by year
                    let year_str = arg.to_string();
                    self.cars
                        .iter()
                        .find(|car| car.year == year_str)
                        .map_or_else(
                            || {
                                "Constructors' Champion not found for the specified year."
                                    .to_string()
                            },
                            |car| {
                                format!(
                                    "Year: {}, Constructors' Champion: {}",
                                    car.year, car.constructor
                                )
                            },
                        )
                }
                _ => {
                    // Query by constructor name
                    let constructor_name_query = args[1..].join(" ").to_lowercase();
                    let constructor_name_formatted = constructor_name_query
                        .split_whitespace()
                        .map(|name_part| {
                            name_part
                                .char_indices()
                                .map(|(i, c)| {
                                    if i == 0 {
                                        c.to_uppercase().to_string()
                                    } else {
                                        c.to_string()
                                    }
                                })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    let years_list: Vec<String> = self
                        .cars
                        .iter()
                        .filter(|car| car.constructor.to_lowercase() == constructor_name_query)
                        .map(|car| car.year.clone())
                        .collect();

                    if years_list.is_empty() {
                        format!(
                            "No Constructors' championship found for '{}'.",
                            constructor_name_formatted
                        )
                    } else {
                        format!(
                            "{} won the Constructors' Championship in: {}",
                            constructor_name_formatted,
                            years_list.join(", ")
                        )
                    }
                }
            }
        } else {
            "Please provide a year or a full constructor name.".to_string()
        };

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    }

    // Checking if someone gave the correct answer to the question
    pub async fn process_answer(&self, ctx: &Context, msg: &Message) {
        let mut current_question = self.current_question.lock().await;

        if let Some(ref question) = *current_question {
            if let Some(answer) = self.questions.get(question) {
                if msg.content.to_lowercase() == answer.to_lowercase() {
                    let mut points = self.user_points.lock().await;
                    let user_points = points.entry(msg.author.id).or_insert(0);
                    *user_points += 1;

                    let response = if *user_points == 5 {
                        format!("Correct answer! Congratulations {}, you are now the F1 Quiz World Champion!", msg.author.name)
                    } else {
                        "Correct answer!".to_string()
                    };

                    let _ = msg.channel_id.say(&ctx.http, &response).await;

                    if *user_points == 5 {
                        let mut quiz_wins = self.quiz_wins.lock().await;
                        *quiz_wins.entry(msg.author.id).or_insert(0) += 1;
                        points.clear();
                    }

                    *self.question_answered.lock().await = true;
                    *current_question = None;
                }
            }
        }
    }

    // Processing the !points command
    pub async fn process_points_command(&self, ctx: &Context, msg: &Message) {
        let points = self.user_points.lock().await;
        let mut points_vec: Vec<(&UserId, &u32)> = points.iter().collect();
        points_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut leaderboard = String::new();
        for (user_id, &score) in points_vec.iter() {
            let user_name = user_id
                .to_user(&ctx.http)
                .await
                .map_or_else(|_| "Unknown User".to_string(), |user| user.name);
            leaderboard.push_str(&format!("{}: {}\n", user_name, score));
        }

        let leaderboard_message = if leaderboard.is_empty() {
            "No one has any points yet.".to_string()
        } else {
            format!("Points Leaderboard:\n{}", leaderboard)
        };

        let _ = msg.channel_id.say(&ctx.http, &leaderboard_message).await;
    }

    // Processing the !leaderboard command
    pub async fn process_leaderboard_command(&self, ctx: &Context, msg: &Message) {
        let quiz_wins = self.quiz_wins.lock().await;
        let mut wins_vec: Vec<(&UserId, &u32)> = quiz_wins.iter().collect();
        wins_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut leaderboard = String::new();
        for (user_id, &win_count) in wins_vec.iter() {
            let user_name = user_id
                .to_user(&ctx.http)
                .await
                .map_or_else(|_| "Unknown User".to_string(), |user| user.name);
            leaderboard.push_str(&format!("{}: {} wins\n", user_name, win_count));
        }

        let leaderboard_message = if leaderboard.is_empty() {
            "No one has won the quiz yet.".to_string()
        } else {
            format!("Quiz Leaderboard:\n{}", leaderboard)
        };

        let _ = msg.channel_id.say(&ctx.http, &leaderboard_message).await;
    }
}

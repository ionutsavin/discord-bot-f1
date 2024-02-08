use csv::ReaderBuilder;
use rand::seq::{IteratorRandom, SliceRandom};
use serde::Deserialize;
use serenity::{
    all::CreateMessage,
    model::id::{ChannelId, UserId},
    prelude::*,
};
use std::env;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io,
    sync::Mutex,
    time::{self, Duration},
};

#[derive(Deserialize)]
pub struct Champion {
    pub year: String,
    pub driver: String,
}

#[derive(Deserialize)]
pub struct Car {
    pub year: String,
    pub constructor: String,
}

pub struct Handler {
    pub quotes: Vec<String>,
    pub current_question: Arc<Mutex<Option<String>>>,
    pub question_answered: Arc<Mutex<bool>>,
    pub questions: HashMap<String, String>,
    pub user_points: Arc<Mutex<HashMap<UserId, u32>>>,
    pub champions: Vec<Champion>,
    pub cars: Vec<Car>,
    pub quiz_wins: Arc<Mutex<HashMap<UserId, u32>>>,
}

impl Handler {
    // Function to load quotes
    pub async fn load_quotes() -> io::Result<Vec<String>> {
        let file = std::fs::read_to_string("quotes.txt")?;
        Ok(file.lines().map(String::from).collect())
    }

    // Function to load questions
    pub async fn load_questions() -> io::Result<HashMap<String, String>> {
        let file = std::fs::File::open("f1_trivia.csv")?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut questions = HashMap::new();

        for result in rdr.records() {
            let record = result?;
            if record.len() == 2 {
                questions.insert(record[0].to_string(), record[1].to_string());
            }
        }
        Ok(questions)
    }

    // Function to load champions
    pub async fn load_champions() -> io::Result<Vec<Champion>> {
        let file = std::fs::File::open("f1_champions.csv")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut champions = Vec::new();

        for result in rdr.deserialize() {
            let champion: Champion = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            champions.push(champion);
        }
        Ok(champions)
    }

    // Function to load cars
    pub async fn load_cars() -> io::Result<Vec<Car>> {
        let file = std::fs::File::open("f1_constructors.csv")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut cars = Vec::new();

        for result in rdr.deserialize() {
            let car: Car = result.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            cars.push(car);
        }
        Ok(cars)
    }

    pub async fn new() -> io::Result<Self> {
        let quotes = Self::load_quotes().await?;
        let questions = Self::load_questions().await?;
        let champions = Self::load_champions().await?;
        let cars = Self::load_cars().await?;

        Ok(Self {
            quotes,
            current_question: Arc::new(Mutex::new(None)),
            question_answered: Arc::new(Mutex::new(true)),
            questions,
            user_points: Arc::new(Mutex::new(HashMap::new())),
            champions,
            cars,
            quiz_wins: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    // Function to get a random quote
    pub fn get_random_quote(&self) -> Option<&String> {
        self.quotes.choose(&mut rand::thread_rng())
    }

    // Function to get the channel id
    pub fn get_channel_id() -> u64 {
        let channel_id_str =
            env::var("DISCORD_CHANNEL_ID").expect("Expected a channel ID in the environment");
        channel_id_str.parse::<u64>().expect("Invalid channel ID")
    }

    // Function to ask a question when the bot starts and then every 60 seconds
    pub async fn ask_question(
        ctx: Context,
        current_question: Arc<Mutex<Option<String>>>,
        question_answered: Arc<Mutex<bool>>,
        questions: HashMap<String, String>,
    ) {
        let mut interval = time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let mut q_answered = question_answered.lock().await;
            if *q_answered {
                let question = questions.keys().choose(&mut rand::thread_rng()); // Select a random question
                if let Some(q) = question {
                    *q_answered = false; // Set to false as soon as a new question is chosen

                    let mut curr_question = current_question.lock().await;
                    *curr_question = Some(q.to_string());

                    let channel_id = ChannelId::new(Self::get_channel_id());
                    let builder = CreateMessage::new().content(q);
                    if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }
}

use std::cmp::Ordering;
use std::io;
use rand::Rng;
use std::collections::HashMap;
use crate::GuessMessages::{EQUAL, GREATER, LESS};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    log("--- GUESS GAME ---");

    loop {
        let input_u32 = get_input();
        let guess_result: bool = guess(input_u32, secret_number);
        if guess_result { break }
    }
}

// Didn`t understand yet if it`s better to work with String or &str
fn log(log_str: &str) {
    let date_now = chrono::offset::Utc::now().to_string();
    println!("{date_now} - {log_str}");
}

fn guess(guess_number: u32, secret: u32) -> bool {
    let messages = get_messages_dict();
    log(guess_number.to_string().as_str());

    // Not quite understand the need for the & operator (may have something to do with de #derive)
    match guess_number.cmp(&secret) {
        Ordering::Less => { log(messages[&LESS].as_str()); false },
        Ordering::Greater => { log(messages[&GREATER].as_str()); false },
        Ordering::Equal => { log(messages[&EQUAL].as_str()); true },
    }
}

fn get_input() -> u32 {
    let mut input_str = String::new();
    log("Please enter a number: ");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Error on the input");

    let input_u32: u32 = input_str
        .trim()
        .parse()
        .expect("Error parsing input to Number value");

    input_u32
}

#[derive(Eq, PartialEq, Hash)]
enum GuessMessages {
    LESS,
    GREATER,
    EQUAL,
}

fn get_messages_dict() -> HashMap<GuessMessages, String> {
    HashMap::from([
        (LESS, String::from("The guess number is lower then the secret")),
        (GREATER, String::from("The guess number is greater then the secret")),
        (EQUAL, String::from("Success!!!")),
    ])
}

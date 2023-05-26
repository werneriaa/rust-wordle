use chrono::{prelude::*, Duration};
use colored::*;
use rand::prelude::SliceRandom;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
    Features missing:
    - Tests
    - Put the code into multiple files
    - Packing the thing into a container
*/

fn main() {
    println!("Welcome to Wordle!");
    let args: Vec<String> = env::args().collect();
    let no_limit: bool = check_for_no_limit(&args);
    let random_word: bool = check_for_random_word(&args);

    let words: Vec<String> =
        read_words_from_file("./../output.txt").expect("Failed to get a words.");
    let hidden_word: String;

    if random_word {
        hidden_word = pick_random_word(&words).expect("Failed to get hidden word");
    } else {
        hidden_word = pick_word_for_this_day(&words).expect("Failed to get hidden word");
    }

    let mut guessed_words: Vec<String> = vec![];

    while guessed_words.len() < 5 || no_limit {
        if no_limit {
            println!("Guess the word (Infinite guesses remaining):")
        } else {
            let guesses_left: usize = 5 - guessed_words.len();
            println!("Guess the word ({} guesses remaining):", guesses_left);
        }
        let mut user_input: String = String::new();
        let guessed_word: String;
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");

        guessed_word = user_input.trim().to_lowercase();

        /*

            This could be improved.
            Maybe a validity check functions ..?

        */

        if guessed_word.len() < 5 {
            println!("{} - Your word is too short!", guessed_word.bright_red());
            continue;
        }

        if guessed_word.len() > 5 {
            println!("{} - Your word is too long!", guessed_word.bright_red());
            continue;
        }

        if !is_real_word(&words, &guessed_word) {
            println!(
                "{} - Invalid word, try another one!",
                guessed_word.bright_red()
            );
            continue;
        }

        if guessed_words.contains(&guessed_word.to_string()) {
            println!(
                "{} - You already guessed this word, try another one",
                guessed_word.bright_red()
            );
            continue;
        }

        if guessed_word == hidden_word {
            println!("{}", guessed_word.green());
            println!("Congratulations! You guessed the word!");
            break;
        } else {
            let mut colored_word: String = String::new();
            for (guessed_char, hidden_char) in guessed_word.chars().zip(hidden_word.chars()) {
                if guessed_char == hidden_char {
                    colored_word.push_str(&guessed_char.to_string().bright_green().to_string());
                } else if hidden_word.contains(guessed_char) {
                    colored_word.push_str(&guessed_char.to_string().bright_yellow().to_string());
                } else {
                    colored_word.push_str(&guessed_char.to_string());
                }
            }
            println!("{}", colored_word);
            guessed_words.push(guessed_word)
        }
    }

    if guessed_words.len() == 5 || no_limit {
        println!("Game over! You ran out of guesses.");
        println!("The correct word was: {}", hidden_word.bright_cyan());
    }
}

fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .map(|line: Result<String, io::Error>| line.unwrap())
        .collect();
    Ok(words)
}

fn pick_random_word(words: &[String]) -> Option<String> {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    words.choose(&mut rng).cloned()
}

fn is_real_word<'a>(words: &'a [String], guessed_word: &str) -> bool {
    for word in words {
        if word == guessed_word {
            return true;
        }
    }
    false
}

fn pick_word_for_this_day(words: &Vec<String>) -> Option<String> {
    let utc: DateTime<Utc> = Utc::now();
    let mut formatted_utc: DateTime<Utc> = utc;
    formatted_utc = formatted_utc + Duration::days(1);
    formatted_utc = formatted_utc - Duration::seconds(utc.second().into());
    formatted_utc = formatted_utc - Duration::minutes(utc.minute().into());
    formatted_utc = formatted_utc - Duration::hours(utc.hour().into());
    formatted_utc = formatted_utc - Duration::nanoseconds(utc.nanosecond().into());
    let index: usize = formatted_utc.timestamp() as usize % words.len();
    return words.get(index).cloned();
}

fn check_for_no_limit(params: &[String]) -> bool {
    let test_string: String = "--no-limit".to_string();
    return params.contains(&test_string);
}

fn check_for_random_word(params: &[String]) -> bool {
    let test_string: String = "--random".to_string();
    return params.contains(&test_string);
}

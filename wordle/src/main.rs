use chrono::{prelude::*, Duration};
use colored::*;
use rand::prelude::SliceRandom;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
    Features missing:
    - Packing the thing into a container
*/

fn main() {
    println!("Welcome to Wordle!");
    let args: Vec<String> = env::args().collect();
    let no_limit: bool = check_for_argument(&args, "--no-limit");
    let random_word: bool = check_for_argument(&args, "--random");

    let words: Vec<String> = read_words_from_file("./output.txt").expect("Failed to get words.");
    let hidden_word: String = if random_word {
        pick_random_word(&words).expect("Failed to get hidden word")
    } else {
        pick_word_for_this_day(&words).expect("Failed to get hidden word")
    };

    let mut guessed_words: Vec<String> = Vec::new();

    while guessed_words.len() < 5 || no_limit {
        if no_limit {
            println!("Guess the word (Infinite guesses remaining):");
        } else {
            let guesses_left: usize = 5 - guessed_words.len();
            println!("Guess the word ({} guesses remaining):", guesses_left);
        }

        let user_input = read_user_input().expect("Failed to read input.");
        let guessed_word = user_input.trim().to_lowercase();

        if is_guess_valid(&guessed_word, &words, &guessed_words) {
            if guessed_word == hidden_word {
                println!("{}", guessed_word.green());
                println!("Congratulations! You guessed the word!");
                break;
            } else {
                let colored_word = guessed_word
                    .chars()
                    .zip(hidden_word.chars())
                    .map(|(guessed_char, hidden_char)| {
                        if guessed_char == hidden_char {
                            guessed_char.to_string().bright_green().to_string()
                        } else if hidden_word.contains(guessed_char) {
                            guessed_char.to_string().bright_yellow().to_string()
                        } else {
                            guessed_char.to_string()
                        }
                    })
                    .collect::<String>();
                println!("{}", colored_word);
                guessed_words.push(guessed_word);
            }
        }
    }

    if guessed_words.len() == 5 {
        println!("Game over! You ran out of guesses.");
        println!("The correct word was: {}", hidden_word.bright_cyan());
    }
}

fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);

    reader.lines().collect::<io::Result<Vec<String>>>()
}

fn pick_random_word(words: &[String]) -> Option<String> {
    let mut rng = rand::thread_rng();
    words.choose(&mut rng).cloned()
}

fn is_real_word(words: &[String], guessed_word: &str) -> bool {
    words.contains(&guessed_word.to_string())
}

fn pick_word_for_this_day(words: &[String]) -> Option<String> {
    let utc: DateTime<Utc> = Utc::now();
    let formatted_utc = utc
        + Duration::days(1)
        - Duration::seconds(utc.second() as i64)
        - Duration::minutes(utc.minute() as i64)
        - Duration::hours(utc.hour() as i64)
        - Duration::nanoseconds(utc.nanosecond() as i64);

    let index: usize = formatted_utc.timestamp() as usize % words.len();
    words.get(index).cloned()
}

fn check_for_argument(args: &[String], arg: &str) -> bool {
    args.iter().any(|a| a == arg)
}

fn is_guess_valid(guessed_word: &str, words: &[String], guessed_words: &[String]) -> bool {
    if guessed_word.len() < 5 {
        println!("{} - Your word is too short!", guessed_word.bright_red());
        return false;
    }

    if guessed_word.len() > 5 {
        println!("{} - Your word is too long!", guessed_word.bright_red());
        return false;
    }

    if !is_real_word(words, guessed_word) {
        println!(
            "{} - Invalid word, try another one!",
            guessed_word.bright_red()
        );
        return false;
    }

    if guessed_words.contains(&guessed_word.to_string()) {
        println!(
            "{} - You already guessed this word, try another one",
            guessed_word.bright_red()
        );
        return false;
    }

    true
}

fn read_user_input() -> io::Result<String> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_lowercase())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_no_limit() {
        // Test when args contains "--no-limit"
        let args = vec!["--no-limit".to_string()];
        assert_eq!(check_for_argument(&args, "--no-limit"), true);

        // Test when args does not contain "--no-limit"
        let args = vec!["other_arg".to_string(), "another_arg".to_string()];
        assert_eq!(check_for_argument(&args, "--no-limit"), false);
    }

    #[test]
    fn test_check_for_random_word() {
        // Test when args contains "--random"
        let args = vec!["--random".to_string()];
        assert_eq!(check_for_argument(&args, "--random"), true);

        // Test when args does not contain "--random"
        let args = vec!["other_arg".to_string(), "another_arg".to_string()];
        assert_eq!(check_for_argument(&args, "--random"), false);
    }
    #[test]
    fn test_is_guess_valid_valid_word() {
        let words = vec![
            String::from("apple"),
            String::from("juice"),
            String::from("grape"),
            String::from("mango"),
            String::from("berry"),
        ];
        let guessed_words = vec![String::from("juice"), String::from("apple")];
        let guessed_word = "grape";

        assert!(is_guess_valid(guessed_word, &words, &guessed_words));
    }

    #[test]
    fn test_is_guess_valid_short_word() {
        let words = vec![
            String::from("apple"),
            String::from("juice"),
            String::from("grape"),
            String::from("mango"),
            String::from("berry"),
        ];
        let guessed_words = vec![String::from("juice"), String::from("apple")];
        let guessed_word = "hey";

        assert!(!is_guess_valid(guessed_word, &words, &guessed_words));
    }

    #[test]
    fn test_is_guess_valid_long_word() {
        let words = vec![
            String::from("apple"),
            String::from("juice"),
            String::from("grape"),
            String::from("mango"),
            String::from("berry"),
        ];
        let guessed_words = vec![String::from("juice"), String::from("apple")];
        let guessed_word = "cherry";

        assert!(!is_guess_valid(guessed_word, &words, &guessed_words));
    }

    #[test]
    fn test_is_guess_valid_invalid_word() {
        let words = vec![
            String::from("apple"),
            String::from("juice"),
            String::from("grape"),
            String::from("mango"),
            String::from("berry"),
        ];
        let guessed_words = vec![String::from("recap"), String::from("apple")];
        let guessed_word = "hey";

        assert!(!is_guess_valid(guessed_word, &words, &guessed_words));
    }

    #[test]
    fn test_is_guess_valid_duplicate_word() {
        let words = vec![
            String::from("apple"),
            String::from("juice"),
            String::from("grape"),
            String::from("mango"),
            String::from("berry"),
        ];
        let guessed_words = vec![String::from("juice"), String::from("apple")];
        let guessed_word = "juice";

        assert!(!is_guess_valid(guessed_word, &words, &guessed_words));
    }
}

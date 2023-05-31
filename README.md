# Wordle - A Terminal-based Word Guessing Game

Wordle is a simple game where the player tries to guess a hidden word by entering guesses and receiving feedback on whether each guess matches the hidden word or not.

## Installation

To run the Wordle game, you'll need to have Rust installed on your system. If you don't have Rust installed already, you can download and install it from the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can download the Wordle source code from the Github repository:

`git clone https://github.com/werneriaa/rust-wordle`

## Usage

To run the Wordle game, navigate to the wordle directory of the downloaded project and run the following command:

This will compile the Rust code and run the Wordle game in the terminal.

`cd wordle`

`cargo build`

`cargo run`

## Game Rules

The objective of the game is to guess the hidden word before running out of turns. The player has five guesses and the game will provide feedback on the correctness of each guess.

The current state of the hidden word with the guessed letters filled in will be displayed, along with a count of the number of turns remaining. If the letter is displayed in yellow, it means the character can be found in the word, but the letter is in the wrong place. If the letter is displayed in green, it means the character is found in the word and is in the correct place.

If the player correctly guesses the word, the game ends. However, if the player fails to guess the word within the five turns, the game ends and the player loses.

## Optinal start up parameters

`cargo run -- --no-limit`

User infinite guesses

`cargo run -- --random`

Uses random word instead of the current day's word

## Running in docker

build image:
`docker build -t wordle:latest .`

run program:

`docker run -it wordle:latest {params}`

## Running tests

Run tests with command:
`cargo test`

## License

The Wordle game is distributed under the MIT license. See the [LICENSE](https://opensource.org/licenses/MIT) file for more details.

## Contributing

If you'd like to contribute to the Rust Wordle game, feel free to submit a pull request or open an issue on the Github repository.

Thanks for playing!

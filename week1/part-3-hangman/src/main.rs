// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    println!("Welcome to CS110L Hangman!");

    let mut chance: u32 = NUM_INCORRECT_GUESSES;
    let mut word: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut cnt: usize = 0;
    let mut guess: Vec<char> = Vec::new();
    let mut input = String::new();
    let mut input_char: char;
    let mut wrong: bool;

    while chance > 0 {
        if cnt == secret_word.len() {
            break;
        }

        println!("The word so far is {}", word.iter().collect::<String>());
        println!("You have guessed the following letters: {}", guess.iter().collect::<String>());
        println!("You have {} guesses left", chance);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line.");
        input_char = input.chars().nth(0).expect("");

        chance -= 1;
        wrong = true;
        for i in 0..secret_word_chars.len() {
            if secret_word_chars[i] == input_char {
                word[i] = secret_word_chars[i];
                secret_word_chars[i] = '-';
                chance += 1;
                cnt += 1;
                wrong = false;
                break;
            }
        }
        if wrong {
            println!("Sorry, that letter is not in the word");
        }
        
        guess.push(input_char);
        println!(); 
    }

    if chance > 0 {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    } else {
        println!("Sorry, you ran out of guesses!");
    }

    return;
}

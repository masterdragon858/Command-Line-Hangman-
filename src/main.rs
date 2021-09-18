use rand::Rng;
use std::io;

struct HangmanWord {
    name: String,
    description: String,
}

impl HangmanWord {
    fn new(name: &str, description: &str) -> HangmanWord {
        HangmanWord {
            name: String::from(name),
            description: String::from(description),
        }
    }
}

struct HangmanGameState {
    chances: i32,
    hangman_picture: String,
}

impl HangmanGameState {
    fn new(chances: i32) -> HangmanGameState {
        HangmanGameState {
            chances,
            hangman_picture: String::from(
                "
           |-  O
           |  -|-
           |  /|",
            ),
        }
    }

    fn update_hangman_picture(&mut self) {
        match self.chances {
            5 => {
                self.hangman_picture = String::from(
                    "
           |-  O
           |  -|
           |  /|",
                )
            }
            4 => {
                self.hangman_picture = String::from(
                    "
           |-  O
           |   |
           |  /|",
                )
            }
            3 => {
                self.hangman_picture = String::from(
                    "
           |-  O
           |   |
           |  /",
                )
            }
            2 => {
                self.hangman_picture = String::from(
                    "
           |-  O
           |   |
           |  ",
                )
            }
            1 => {
                self.hangman_picture = String::from(
                    "
           |-  O
           |  
           |  ",
                )
            }
            _ => (),
        }
    }
}

fn main() {
    let default_word: HangmanWord =
        HangmanWord::new("default", "If you are seeing this something went wrong");
    let mut game_state: HangmanGameState = HangmanGameState::new(6);

    let mut playing: bool = true;

    let hangman_words_vec_food: Vec<HangmanWord> = vec![
        HangmanWord::new(
            "pizza",
            "A popular italian dish consisting of dough, tomato sauce, and cheese",
        ),
        HangmanWord::new("burger", "Popular american food, usually served with fries"),
    ];

    let hangman_words_vec_occupations: Vec<HangmanWord> = vec![HangmanWord::new(
        "programmer",
        "Best job hands down, ppl who do it are usually amzing too",
    )];

    let hangman_words_vec_miscellanous: Vec<HangmanWord> = vec![
        HangmanWord::new("rocket", "A machine used to conduct outer space travel"),
        HangmanWord::new(
            "sock",
            "A cloth piece of clothing that ordanarily goes on the feet",
        ),
    ];

    while playing {
        let mut topic_choice: String = String::new();

        println!(
            "What topic would oyu like to pick? Food, Occupations, or Miscellanous? (F, O, or M) "
        );

        io::stdin()
            .read_line(&mut topic_choice)
            .expect("Something went wrong getting your topic chocie");

        let mut current_word: &HangmanWord = &default_word;

        if topic_choice.trim() == "F" {
            current_word = &hangman_words_vec_food
                [rand::thread_rng().gen_range(0..hangman_words_vec_food.len())];
        } else if topic_choice.trim() == "O" {
            current_word = &hangman_words_vec_occupations
                [rand::thread_rng().gen_range(0..hangman_words_vec_occupations.len())];
        } else if topic_choice.trim() == "M" {
            current_word = &hangman_words_vec_miscellanous
                [rand::thread_rng().gen_range(0..hangman_words_vec_miscellanous.len())];
        } else {
            println!("That is not a valid topic chocie");
            continue;
        }

        println!("{}", current_word.description);
        let mut unknown_word: String = String::new();
        for _letter in current_word.name.chars() {
            unknown_word.push_str("_ ");
        }

        loop {
            game_state.update_hangman_picture();
            println!("{}", game_state.hangman_picture);
            println!("Unknown Word: {}", unknown_word);

            let mut letter_guess: String = String::new();

            io::stdin()
                .read_line(&mut letter_guess)
                .expect("Something went wrong reading your input");

            if letter_guess.trim().len() > 1 {
                println!("Your guess should only be one letter");
                continue;
            } else {
                let letter_guess_char: char = letter_guess.chars().next().unwrap();
                if current_word.name.contains(letter_guess_char) {
                    for i in 0..current_word.name.len() {
                        if current_word
                            .name
                            .chars()
                            .nth(i)
                            .expect("Something went wrong")
                            == letter_guess_char
                        {
                            unknown_word.remove(i + i);
                            unknown_word.insert(i + i, letter_guess_char)
                        }
                    }
                } else {
                    game_state.chances -= 1;
                    println!(
                        "That letter is not in the word. Guesses left {}",
                        game_state.chances
                    );
                }
            }

            let whitespaceless_unknownword: String = unknown_word.split_whitespace().collect();
            if whitespaceless_unknownword == current_word.name {
                println!("{}", unknown_word);
                println!("You win! You guessed the word!");
                break;
            }

            if game_state.chances == 0 {
                println!("You've lost all your chances!");
                break;
            }
        }

        let mut play_again: String = String::new();
        println!("Would you like to play again?");

        io::stdin()
            .read_line(&mut play_again)
            .expect("Something went wrong reading if you want to play again");

        if play_again.to_lowercase().trim() == "no" {
            println!("Hope you had fun playing!");
            break;
        } else if play_again.to_lowercase().trim() == "yes" {
            println!("Ok get ready!");
        } else {
            println!("Something went wrong reading your input, try again");
            loop {
                let mut play_again: String = String::new();
                println!("Would you like to play again?");

                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Something went wrong reading if you want to play again");

                if play_again.to_lowercase().trim() == "no" {
                    println!("Hope you had fun playing!");
                    playing = false;
                    break;
                } else if play_again.to_lowercase().trim() == "yes" {
                    println!("Ok get ready!");
                    break;
                } else {
                    println!("Something went wrong reading your input, try again");
                }
            }
        }
    }
}

fn main() {
    let mut hg: Hangman = Hangman::new();
    hg.run();
}

enum GeussStatus {
    AlreadyGuessed,
    RightGuess,
    WrongGuess
}

#[derive(Clone)]
struct Hangman{
    word: String,
    wrong_guesses: String,  //String = Vec<u8> but with needed function contains()
    right_guesses: String,
    lives: u32
}

impl Hangman{
    pub fn new() -> Hangman {
        //TODO: get word and load it into the game
        let w = "apple";
        Hangman {
            word: String::from(w),
            right_guesses: String::new(),
            wrong_guesses: String::new(),
            lives: 6
        }
    }

    fn run(&mut self) {
        //loop until no lives:
        // 1. generate word
        // 2. update and display word
        // 3. prompt/get guess
        // 4. check guess & update hangman staus/lives (if right, win?)
                    
        //TODO: generate word
        while self.lives > 0 {
            println!("{}", format_masked_list(&self.word, &self.right_guesses));
            println!("Enter your guess:");
            let guess = get_letter();
            match self.check_guess(&guess) {
                GeussStatus::AlreadyGuessed => println!("You have already guessed \'{}\'", &guess),
                GeussStatus::RightGuess => {
                    self.right_guesses.push(guess.chars().next().unwrap());
                    println!("correct guess");
                    if self.is_won() { 
                        println!("yo win! the word was: {}", self.word); 
                        std::process::exit(0); //Or prompt new game
                    }
                },
                GeussStatus::WrongGuess => {
                    self.lives -= 1;
                    self.wrong_guesses.push(guess.chars().next().unwrap());
                    println!("incorrect guess");
                }
            }
        }
        println!("Game Over. You Lose");
    }

    fn check_guess(&mut self, c: &str) -> GeussStatus {
        if self.right_guesses.contains(&c) || self.wrong_guesses.contains(&c) {
            return GeussStatus::AlreadyGuessed
        }
        else if self.word.contains(c) {
            return GeussStatus::RightGuess
        }

        GeussStatus::WrongGuess
    }

    fn is_won(&self) -> bool {
        for c in self.word.chars() {
            if !self.right_guesses.contains(c) { return false; }
        }
        true
    }
    
}


fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("plz enter something");

    //Strip endline characters
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);

    /* Alternate way to strip endline characters
    fn trim_newline(s: &mut String) {
        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        }
    }   
    */

    input
}

// get a single lowercase letter as a String
fn get_letter() -> String {
    let input = get_input();
    if input.len() == 1 && input.chars().next().unwrap().is_alphabetic() {
        return input.to_lowercase();
    }
    println!("input a single letter");
    get_letter()
}

fn get_char() -> char {
    let mut c = get_input();
    if c.chars().count() != 1 { 
        println!("not a letter");
        c = get_input();
    }
    return c.chars().nth(0).unwrap();
}

fn format_masked_list(word: &str, mask: &str) -> String {
    let mut result: String = String::new();

    for c in word.chars() {
        result.push(
            if mask.contains(c) {c}
            else {'_'}
        );
        result.push(' ');
    }

    result
}
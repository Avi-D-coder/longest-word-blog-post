use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const BANNED_LETTERS: [char; 8] = ['g', 'k', 'm', 'q', 'v', 'w', 'x', 'z'];

fn main() {
    let words = BufReader::new(File::open("./words.txt").expect("file does not exist")).lines();

    // fn is_letter_allowed(letter: char) -> bool {
    //     BANNED_LETTERS.into_iter().all(|banned| letter != *banned)
    // }
    let is_letter_allowed = |letter| BANNED_LETTERS.into_iter().all(|banned| letter != *banned);

    let longest_word = words
        .map(|word| word.expect("bad word encoding"))
        .filter(|word| word.chars().all(is_letter_allowed))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .expect("no words found");

    println!("{}", longest_word);
}

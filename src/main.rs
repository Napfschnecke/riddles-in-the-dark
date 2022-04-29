extern crate base64;

use colored::*;

static RIDDLE_FILE: &'static str = include_str!("../riddles.txt");
static SOLUTION_FILE: &'static str = include_str!("../solutions.txt");
static TITLE_ART: &'static str = include_str!("../title.txt");
static CONGRATS_ART: &'static str = include_str!("../congrats.txt");

const CORRECT: &str = "Das ist die richtige Antwort! 🙂";
const INCORRECT: &str = "ist nicht die richtige Antwort 🤔";

fn main() {
    use std::io::{stdin,stdout,Write};
    control::set_virtual_terminal(true).unwrap();

    let riddles = read_riddles();
    let solutions = read_solutions();
    let mut counter = 0;
    let mut print_riddle = true;

    println!("{}", TITLE_ART.bright_red());

    while counter < riddles.len() {
        let current_riddle = &riddles[counter];
        let mut s=String::new();

        if print_riddle { transform_riddle(counter, current_riddle, riddles.len()) };

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        if solutions[counter] == s {
            counter = counter + 1;
            if counter == riddles.len() {
                print_final();
            } else {
                println!("\n{}\n", CORRECT.bright_green());
                print_riddle = true;
            }
        } else {
            println!("\n{} {}\n", s.bright_magenta().bold(), INCORRECT.bright_red());
            print_riddle = false;
        }
    }
}

fn read_riddles() -> Vec<String>{
    RIDDLE_FILE.lines().map(|line| line.to_string()).collect()
}

fn read_solutions() -> Vec<String> {
    SOLUTION_FILE.lines().map(|line| line.to_string()).collect()
}

fn transform_riddle(counter: usize, riddle: &String, num_riddles: usize) {
    match counter {
        1 => println!("{}|{}: {} ....... huch. Was ist denn hier basiert?", counter + 1, num_riddles, base64::encode(riddle).bright_yellow()),
        19 => println!("{}|{}: {}", counter + 1, num_riddles, riddle.chars().rev().collect::<String>().bright_yellow()),
        _ => println!("{}|{}: {}", counter + 1, num_riddles, riddle.bright_yellow()),
    }
}

fn print_final() {
    println!("{}", CONGRATS_ART.bright_green());
    let final_riddle = " Fips und Alica sitzen aufm Baum ";
    let top = (0..final_riddle.len() + 12).map(|_| "#").collect::<String>().bright_green();
    let side = "######".bright_green();
    println!("\n\n{}", top);
    println!("{}", top);
    println!("{}{}{}", side, final_riddle.bright_yellow().bold(), side);
    println!("{}", top);
    println!("{}\n\n", top);
}

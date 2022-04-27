extern crate base64;

use colored::Colorize;

static RIDDLE_FILE: &'static str = include_str!("../riddles.txt");
static SOLUTION_FILE: &'static str = include_str!("../solutions.txt");
const CORRECT: &str = "Das ist die richtige Antwort! 🙂";
const INCORRECT: &str = "ist nicht die richtige Antwort 🤔";

fn main() {
    use std::io::{stdin,stdout,Write};
    let riddles = read_riddles();
    let solutions = read_solutions();
    let mut counter = 0;
    let mut print_riddle = true;

    while counter < riddles.len() {
        let current_riddle = &riddles[counter];
        let mut s=String::new();

        if print_riddle { transform_riddle(counter, current_riddle) };

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

fn transform_riddle(counter: usize, riddle: &String) {
    match counter {
        1 => println!("{} ....... huch. Was ist denn hier basiert?", base64::encode(riddle)),
        19 => println!("{}", riddle.chars().rev().collect::<String>()),
        _ => println!("{}", riddle),
    }
}

fn print_final() {
    let final_riddle = " Fips und Alica sitzen aufm Baum ";
    let top = (0..final_riddle.len() + 6).map(|_| "#").collect::<String>().bright_green();
    let side = "###".bright_green();
    println!("\n\n{}", top);
    println!("{}", top);
    println!("{}{}{}", side, final_riddle.bright_yellow().bold(), side);
    println!("{}", top);
    println!("{}\n\n", top);
}

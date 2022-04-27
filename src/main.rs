extern crate base64;
static RIDDLE_FILE: &'static str = include_str!("../riddles.txt");
static SOLUTION_FILE: &'static str = include_str!("../solutions.txt");
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
            println!("{} ist die richtige Antwort!",s);
            counter = counter + 1;
            print_riddle = true;
        } else {
            println!("{} ist nicht die richtige Antwort...",s);
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
        1 => print!("{} ....... huch. Was ist denn hier basiert?\t", base64::encode(riddle)),
        19 => print!("{}\t", riddle.chars().rev().collect::<String>()),
        _ => print!("{}\t", riddle),
    }
}

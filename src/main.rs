
static RIDDLE_FILE: &'static str = include_str!("../riddles.txt");
static SOLUTION_FILE: &'static str = include_str!("../solutions.txt");
fn main() {
    use std::io::{stdin,stdout,Write};
    let riddles = read_riddles();
    let solutions = read_solutions();
    let mut counter = 0;
    while counter < riddles.len() {
        let mut s=String::new();
        print!("{}: ", riddles[counter]);
        let _=stdout().flush();
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
        } else {
            println!("{} ist nicht die richtige Antwort...",s);
        }
    }
}

fn read_riddles() -> Vec<String>{
    RIDDLE_FILE.lines().map(|line| line.to_string()).collect()
}

fn read_solutions() -> Vec<String> {
    SOLUTION_FILE.lines().map(|line| line.to_string()).collect()
}

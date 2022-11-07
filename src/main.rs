use rand::Rng;
use std::io::stdin;
//TODO implement score tracking
//TODO implement blazingly fast time tracking

struct Hands {
    rock: &str,
    paper: &str,
    scissors: &str,
}

impl 

const DOTTED_LINE: &str = "----------------------------";
fn main() {

    println!("Welcome to 🚀RPS🚀\n");
    loop {
        println!("What do you want to play?\n{DOTTED_LINE}");
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: String = String::from(input.to_uppercase().trim());
        let comp_pick: String = computer_hand();
        println!(
            "\nConfirmed input as: [{}🚀]\nComputer has chosen: [{comp_pick}🤖]\n\nOutcome: {}\n{DOTTED_LINE}",
            input,
            will_it_win(&input, &comp_pick)
        );
    }
}

fn computer_hand() -> String {
    let comp_hand: u8 = rand::thread_rng().gen_range(1..=3);
    match comp_hand {
        1 => String::from("ROCK"),
        2 => String::from("PAPER"),
        3 => String::from("SCISSORS"),
        _ => unreachable!(),
    }
}

const WIN: &str = "You win! 🚀🤑🚀";
const EVEN: &str = "It's even! 😐🤨😴";
const LOSS: &str = "You lose! 💀😭🤬";

fn will_it_win(hand: &String, opp_hand: &String) -> String {
    // Check struct for hand key
    // if opp_hand is the same as hand key value return win
    // if opp_hand = hand return tie
    // else return loose
    let struct_value = match hand {
        "ROCK" => ,
        "PAPER" =>,
        "SCISSORS" =>,
    }
}

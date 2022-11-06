use rand::Rng;
use std::io::stdin;
//TODO implement lives
//TODO implement score tracking
//TODO implement blazingly fast time tracking
//TODO maybe remove unecessary enum usage?
enum Hands {
    ROCK,
    PAPER,
    SCISSORS,
}

fn main() {
    const DOTTED_LINE: &str = "----------------------------";
    let mut lives: u8 = 3; //TODO

    println!("Welcome to 🚀RPS🚀\nYou'll start with {lives} lives.");
    loop {
        println!("What do you want to play?\n{DOTTED_LINE}");
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let hand: Hands = match input.trim().to_uppercase().as_str() {
            "ROCK" => Hands::ROCK,
            "PAPER" => Hands::PAPER,
            "SCISSORS" => Hands::SCISSORS,
            _ => unimplemented!(),
        };
        let comp_pick: String = computer_hand();
        println!(
            "\nConfirmed input as: [{}]\nComputer has chosen: [{comp_pick}]\n\nOutcome: {}\n{DOTTED_LINE}",
            value_of_hand(hand),
            will_it_win(input.trim().to_uppercase(), comp_pick.clone())
        );
    }
}

fn value_of_hand(hum_hand: Hands) -> String {
    match hum_hand {
        Hands::ROCK => String::from("ROCK"),
        Hands::PAPER => String::from("PAPER"),
        Hands::SCISSORS => String::from("SCISSORS"),
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

fn will_it_win(hand: String, opp_hand: String) -> String {
    const WIN: &str = "You win! 🤑";
    const EVEN: &str = "It's even! 😐";
    const LOSS: &str = "You lose! 💀";

    let condition: String = match hand.as_str() {
        "ROCK" => match opp_hand.as_str() {
            "ROCK" => String::from(EVEN),
            "PAPER" => String::from(LOSS),
            "SCISSORS" => String::from(WIN),
            _ => unimplemented!(),
        },

        "PAPER" => match opp_hand.as_str() {
            "ROCK" => String::from(WIN),
            "PAPER" => String::from(EVEN),
            "SCISSORS" => String::from(LOSS),
            _ => unimplemented!(),
        },

        "SCISSORS" => match opp_hand.as_str() {
            "ROCK" => String::from(LOSS),
            "PAPER" => String::from(WIN),
            "SCISSORS" => String::from(EVEN),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    return condition;
}

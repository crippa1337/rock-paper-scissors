use rand::Rng;
use std::io::stdin;
//TODO implement score tracking
//TODO implement blazingly fast time tracking

fn main() {
    const DOTTED_LINE: &str = "----------------------------";

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
            "\nConfirmed input as: [{}]\nComputer has chosen: [{comp_pick}]\n\nOutcome: {}\n{DOTTED_LINE}",
            input,
            will_it_win(input.clone(), comp_pick.clone())
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

fn will_it_win(hand: String, opp_hand: String) -> String {
    const WIN: &str = "You win! 🤑";
    const EVEN: &str = "It's even! 😐";
    const LOSS: &str = "You lose! 💀";

    let win_or_loss: String = match hand.as_str() {
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
    return win_or_loss;
}

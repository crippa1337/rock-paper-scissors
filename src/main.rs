use clearscreen;
use platform_dirs::AppDirs;
use rand::Rng;
use std::{
    fs::{self, File},
    io::{stdin, BufRead, BufReader, Write},
    thread,
    time::{Duration, Instant},
};

//TODO implement score tracking

// for equality checks between instances
#[derive(PartialEq)]
enum Hands {
    Rock,
    Paper,
    Scissors,
}

// creates two methods using Hands
impl Hands {
    // win method, returns the winning matchup of a hand
    fn win(&self) -> Hands {
        match &self {
            Hands::Rock => Hands::Scissors,
            Hands::Paper => Hands::Rock,
            Hands::Scissors => Hands::Paper,
        }
    }

    // to_string method, returns an equal String to a hand
    fn to_string(&self) -> String {
        match &self {
            Hands::Rock => "ROCK".to_string(),
            Hands::Paper => "PAPER".to_string(),
            Hands::Scissors => "SCISSORS".to_string(),
        }
    }
}

// function to randomly make a hand for the computer, returns a Hands instance
fn computer_hand() -> Hands {
    // matches a random integer from 1 to 3 and returns a random Hands instance
    match rand::thread_rng().gen_range(1..=3) {
        1 => Hands::Rock,
        2 => Hands::Paper,
        3 => Hands::Scissors,
        // out of reach
        _ => unreachable!(),
    }
}

const DOTTED_LINE: &str = "------------------------------------------------";
const WIN: &str = "You win! 🚀🤑🚀";
const EVEN: &str = "It's even! 😐🤨😴";
const LOSS: &str = "You lose! 💀😭🤬";

fn main() {
    let mut wlt: [u32; 3] = [0, 0, 0];

    let stats_dirs = AppDirs::new(Some("rps_crippa"), false).unwrap();
    let path = stats_dirs.state_dir.join("stats.txt");

    if path.exists() {
        let f = File::open(&path).expect("Unable to open stats file");
        let f = BufReader::new(f);
        let mut index = 0;
        for line in f.lines() {
            let line = line.unwrap();
            let line = line.parse::<u32>().unwrap();
            wlt[index] = line;
            index += 1;
        }
    }

    println!(
        "{}\nWelcome to Rock, Paper, Scissors\n🚀 Blazingly Fast Edition 🚀\nInput 'QUIT' to quit & save 😁\nInput 'CLEAR' to clear the screen 😎\nInput 'STATS' to see your stats 📈",
        DOTTED_LINE
    );

    'main_loop: loop {
        let player_pick = loop {
            println!("Input your hand: ROCK🪨, PAPER📃 or SCISSORS✂️\n{DOTTED_LINE}");
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();

            let input = String::from(input.to_uppercase().trim());
            break match input.as_str() {
                "ROCK" => Hands::Rock,
                "PAPER" => Hands::Paper,
                "SCISSORS" => Hands::Scissors,
                "STATS" => {
                    println!(
                        "WINS [{}]\nLOSSES [{}]\nTIES [{}]\n{DOTTED_LINE}",
                        wlt[0], wlt[1], wlt[2]
                    );
                    continue;
                }
                "CLEAR" => {
                    clearscreen::clear().expect("Failed to clear screen");
                    continue;
                }
                "QUIT" => {
                    fs::create_dir_all(&stats_dirs.state_dir).unwrap();
                    let file = File::create(&path).unwrap();
                    write!(&file, "{}\n{}\n{}", wlt[0], wlt[1], wlt[2])
                        .expect("Failed to write to file");

                    println!("Exiting. . .");
                    thread::sleep(Duration::from_secs(1));
                    break 'main_loop;
                }
                _ => {
                    println!(
                        "{} is an invalid input, please try again.\n{DOTTED_LINE}",
                        input
                    );
                    continue;
                }
            };
        };
        // instant variable set to an Instant created for benchmarking purposes
        let instant = Instant::now();
        // assign a random hand to comp_pick
        let comp_pick = computer_hand();

        let result: String;
        // using PartialEq, see if player_pick (Hands) is equal to comp_pick (Hands)
        if player_pick == comp_pick {
            // set result to even since both inputs are the same
            result = String::from(EVEN);
            wlt[2] += 1
        // see if the winning matchup of player_pick is equal to comp_pick
        } else if player_pick.win() == comp_pick {
            // set result to win since the winning matchup is achieved
            result = String::from(WIN);
            wlt[0] += 1
        // only case left is if the player hasn't won or isn't even against the computer, which means it's a loss
        } else {
            // set result to loss
            result = String::from(LOSS);
            wlt[1] += 1
        };

        println!(
            "🚀 [TIME: {:?}] 🚀\nConfirmed pick as: [{}]\nSuperA.I picks:    [{}]\n{result}\n{DOTTED_LINE}",
            // how many seconds has elapsed since the instant Instant was created
            instant.elapsed(),
            player_pick.to_string(),
            comp_pick.to_string()
        );
    }
}

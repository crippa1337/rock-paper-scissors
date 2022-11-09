use platform_dirs::AppDirs;
use rand::Rng;
use std::{
    fs::{self, File},
    io::{stdin, stdout, BufRead, BufReader, Write},
    thread,
    time::{Duration, Instant},
};

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

    fn lose(&self) -> Hands {
        match &self {
            Hands::Rock => Hands::Paper,
            Hands::Paper => Hands::Scissors,
            Hands::Scissors => Hands::Rock,
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

// randomly makes a hand for the computer, returns a Hands instance
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

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn save_data(wlt: [u32; 3]) {
    let stats_dirs = AppDirs::new(Some("rps_crippa"), false).unwrap();
    let path = stats_dirs.state_dir.join("stats.txt");

    fs::create_dir_all(&stats_dirs.state_dir).unwrap();
    let file = File::create(&path).unwrap();
    write!(&file, "{}\n{}\n{}", wlt[0], wlt[1], wlt[2]).expect("Failed to write to file");
}

const DOTTED_LINE: &str = "------------------------------------------------";
const WIN: &str = "You win! 🚀🤑🚀";
const EVEN: &str = "It's even! 😐🤨😴";
const LOSS: &str = "You lose! 💀😭🤬";
fn main() {
    let mut hard_mode = false;
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
        "{}\n             Rock, Paper, Scissors\n          🚀 Blazingly Fast Edition 🚀\n{DOTTED_LINE}\n'STATS' to see your stats 📈😎\n'HARD' to toggle hard mode 😈🤖\n'RESET' to reset stats ♻️🗑️\n'QUIT' or 'EXIT' to close the window✌️😁",
        DOTTED_LINE
    );

    'main_loop: loop {
        let player_pick = loop {
            println!("{DOTTED_LINE}\nROCK🪨, PAPER📃 or SCISSORS✂️ ?\n{DOTTED_LINE}");
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();

            let input = String::from(input.to_uppercase().trim());
            break match input.as_str() {
                "ROCK" => Hands::Rock,
                "PAPER" => Hands::Paper,
                "SCISSORS" => Hands::Scissors,
                "STATS" => {
                    clear();
                    println!("WINS [{}]\nLOSSES [{}]\nTIES [{}]", wlt[0], wlt[1], wlt[2]);
                    continue;
                }
                "QUIT" | "EXIT" => {
                    clear();
                    for i in "Closing. . .".chars() {
                        print!("{i}");
                        stdout().flush().unwrap();
                        thread::sleep(Duration::from_millis(50));
                    }
                    thread::sleep(Duration::from_millis(500));
                    break 'main_loop;
                }
                "RESET" => {
                    clear();
                    for i in "Resetting stats. . .\n".chars() {
                        print!("{i}");
                        stdout().flush().unwrap();
                        thread::sleep(Duration::from_millis(50));
                    }
                    wlt = [0, 0, 0];
                    save_data(wlt);
                    clear();
                    println!("WINS [{}]\nLOSSES [{}]\nTIES [{}]", wlt[0], wlt[1], wlt[2]);
                    continue;
                }
                "HARD" => {
                    clear();
                    for i in "Toggling hard mode . . .\n".chars() {
                        print!("{i}");
                        stdout().flush().unwrap();
                        thread::sleep(Duration::from_millis(50));
                    }
                    hard_mode ^= true;
                    println!("Hard mode set to: {}", hard_mode);
                    continue;
                }
                _ => {
                    clear();
                    println!("{} is an invalid input, please try again.", input);
                    continue;
                }
            };
        };
        // instant variable set to an Instant created for benchmarking purposes
        let instant = Instant::now();
        // assign a random hand to comp_pick

        let mut comp_pick = Hands::Paper;
        if hard_mode == true {
            comp_pick = player_pick.lose();
        } else {
            comp_pick = computer_hand();
        };

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

        save_data(wlt);

        clear();
        println!(
            "🚀 [TIME: {:?}] 🚀\nConfirmed pick as: [{}]\nSuperA.I picks:    [{}]\n{result}",
            // how many seconds has elapsed since the instant Instant was created
            instant.elapsed(),
            player_pick.to_string(),
            comp_pick.to_string()
        );
    }
}

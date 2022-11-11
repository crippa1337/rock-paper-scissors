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

fn winrate(wlt: [u32; 3]) -> f32 {
    let winrate = (wlt[0] as f32 / (wlt[0] + wlt[1]) as f32) * 100.0;
    if winrate.is_nan() {
        return 0.0;
    }
    return winrate;
}

fn slow_print(input: &str, delay: u32, newline: bool) {
    for i in input.chars() {
        print!("{i}");
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(delay as u64));
    }
    if newline {
        println!();
    }
}

fn show_stats(wlt: [u32; 3]) {
    println!(
        "WINS   [{}]\nLOSSES [{}]\nTIES   [{}]\nWR     [{:.2}%]",
        wlt[0],
        wlt[1],
        wlt[2],
        winrate(wlt)
    );
}

fn results(wlt: &mut [u32; 3], player_pick: Hands, hard_mode: bool, testing_mode: bool) {
    // instant variable created for benchmarking purposes
    let instant = Instant::now();

    let comp_pick: Hands;
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

    if !testing_mode {
        println!(
            "🚀 [TIME: {:?}] 🚀\nConfirmed pick as: [{}]\nSuperA.I picks:    [{}]\n{result}",
            // how many seconds has elapsed since the instant Instant was created
            instant.elapsed(),
            player_pick.to_string(),
            comp_pick.to_string()
        );
    }
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
    let mut testing_mode = false;
    let mut testing_iterations: u32 = 0;
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
        "{}\n             Rock, Paper, Scissors\n          🚀 Blazingly Fast Edition 🚀\n{DOTTED_LINE}\n'HELP' or 'INFO' for help 🤔📝\n'STATS' to see your stats 📈😎\n'HARD' to toggle hard mode 😈🤖\n'RESET' to reset stats ♻️🗑️\n'QUIT' or 'EXIT' to close the window✌️😁",
        DOTTED_LINE
    );

    'main_loop: loop {
        let player_pick = loop {
            println!("{DOTTED_LINE}\n(R)OCK🪨, (P)APER📃 or (S)CISSORS✂️ ?\n{DOTTED_LINE}");
            let mut input: String = String::new();
            stdin().read_line(&mut input).unwrap();

            let input = String::from(input.to_uppercase().trim());
            break match input.as_str() {
                "R" | "ROCK" => Hands::Rock,
                "P" | "PAPER" => Hands::Paper,
                "S" | "SCISSORS" => Hands::Scissors,
                "STATS" => {
                    clear();
                    println!(
                        "WINS   [{}]\nLOSSES [{}]\nTIES   [{}]\nWR     [{:.2}%]",
                        wlt[0],
                        wlt[1],
                        wlt[2],
                        winrate(wlt)
                    );
                    continue;
                }
                "QUIT" | "EXIT" => {
                    clear();
                    slow_print("Closing . . .", 10, false);
                    thread::sleep(Duration::from_millis(500));
                    break 'main_loop;
                }
                "RESET" => {
                    clear();
                    slow_print("Resetting stats . . .", 10, true);
                    thread::sleep(Duration::from_millis(500));
                    wlt = [0, 0, 0];
                    save_data(wlt);
                    clear();
                    show_stats(wlt);
                    continue;
                }
                "HARD" => {
                    clear();
                    slow_print("Toggling hard mode . . .", 10, true);
                    hard_mode ^= true;
                    println!("Hard mode set to: {}", hard_mode);
                    continue;
                }
                "INFO" | "HELP" => {
                    clear();
                    println!("'HELP' or 'INFO' for help 🤔📝\n'STATS' to see your stats 📈😎\n'HARD' to toggle hard mode 😈🤖\n'RESET' to reset stats ♻️🗑️\n'QUIT' or 'EXIT' to close the window✌️😁");
                    continue;
                }
                "TEST" => {
                    clear();
                    slow_print("WARNING! Enabling testing mode will purge your stats\nYou cannot cancel until the test is concluded\nInput any key to cancel or 'Y' to continue", 10, true);
                    let mut testing_input = String::new();
                    stdin()
                        .read_line(&mut testing_input)
                        .expect("Failed to get test_input");
                    let testing_input = testing_input.trim().to_uppercase();
                    match testing_input.as_str() {
                        "Y" => {
                            clear();
                            wlt = [0, 0, 0];
                            testing_mode = true;
                            println!("Amount of test iterations?");
                            let mut buffer = String::new();
                            stdin()
                                .read_line(&mut buffer)
                                .expect("Failed to get test_input");
                            testing_iterations = buffer.trim().parse::<u32>().unwrap();
                            break Hands::Rock;
                        }

                        _ => {
                            testing_mode = false;
                            continue;
                        }
                    }
                }
                _ => {
                    clear();
                    println!("{} is an invalid input, please try again.", input);
                    continue;
                }
            };
        };
        if testing_mode {
            let instant = Instant::now();
            for _ in 0..testing_iterations {
                let player_pick = computer_hand();
                results(&mut wlt, player_pick, hard_mode, testing_mode);
            }
            save_data(wlt);
            println!("Test concluded.\nTime elapsed: {:?}", instant.elapsed());
            println!("{DOTTED_LINE}");
            show_stats(wlt);
        } else {
            clear();
            results(&mut wlt, player_pick, hard_mode, testing_mode);
            save_data(wlt);
        }
    }
}

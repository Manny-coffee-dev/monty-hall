use rand::Rng;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Strategy {
    Swap,
    Stay,
}

impl Strategy {
    #[allow(unused_variables)]
    fn from_string(input: &String) -> Result<Strategy, &'static str> {
        match input.as_str() {
            "swap" => Ok(Strategy::Swap),
            "stay" => Ok(Strategy::Stay),
            _ => Err("not a valid strategy"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
}

#[derive(Debug)]
struct Door {
    number: i32,
    prize: bool,
}

pub struct Config {
    pub total_doors: i32,
    pub strategy: Strategy,
    pub plays: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }
        let total_doors = args[1].clone().parse::<i32>().unwrap();
        let strategy = Strategy::from_string(&args[2]).unwrap();
        let plays = args[3].clone().parse::<i32>().unwrap();

        Ok(Config {
            total_doors,
            strategy,
            plays,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("--- Monty Hall Problem Demonstration ---");
    let total_doors = config.total_doors;
    let strategy = config.strategy;
    let plays = config.plays;

    // Play the game
    let mut wins = 0;
    let mut losses = 0;
    let mut count = 0;
    while count < plays {
        let result = play(&strategy, total_doors);
        if result == Outcome::Win {
            wins += 1;
        } else {
            losses += 1;
        }
        count += 1;
    }
    Ok(println!("Wins: {0} | Losses: {1}", wins, losses))
}

fn play(strategy: &Strategy, total_doors: i32) -> Outcome {
    // Generate doors
    let mut doors = generate_doors(total_doors);
    // Chose a door
    let mut chosen_door = rand::thread_rng().gen_range(0, doors.len());
    // Reveal an incorrect door
    doors = reveal_door(doors, chosen_door);
    // Make a play based on strategy
    match strategy {
        Strategy::Swap => chosen_door = choose_new_door(&doors, chosen_door),
        Strategy::Stay => chosen_door = choose_same_door(&doors, chosen_door),
    };
    // Determine the outcome
    if doors[chosen_door].prize {
        return Outcome::Win;
    } else {
        return Outcome::Lose;
    }
}

fn choose_new_door(doors: &Vec<Door>, chosen_door: usize) -> usize {
    let mut new_chosen_door = rand::thread_rng().gen_range(0, doors.len());
    // Make sure we choose a different door
    while new_chosen_door == chosen_door {
        new_chosen_door = rand::thread_rng().gen_range(0, doors.len());
    }
    return new_chosen_door;
}

fn choose_same_door(doors: &Vec<Door>, chosen_door: usize) -> usize {
    // Account for shrinking doors vector
    let mut new_chosen_door = chosen_door;
    if doors.len() == chosen_door {
        new_chosen_door -= 1;
    }
    return new_chosen_door;
}

fn reveal_door(mut doors: Vec<Door>, chosen_door: usize) -> Vec<Door> {
    let mut revealed_door = rand::thread_rng().gen_range(0, doors.len());
    // Make sure we don't remove the chosen door or the door with the prize
    while revealed_door == chosen_door || doors[revealed_door].prize {
        revealed_door = rand::thread_rng().gen_range(0, doors.len());
    }
    doors.remove(revealed_door);
    return doors;
}

fn generate_doors(total_doors: i32) -> Vec<Door> {
    // Create an empty vector to hold the doors
    let mut doors = Vec::<Door>::new();

    // Decide which door contains the prize
    let num = rand::thread_rng().gen_range(0, total_doors);

    // Generate the doors
    let mut count = 0;
    while count < total_doors {
        if count == num {
            doors.push(Door {
                number: count,
                prize: true,
            });
        } else {
            doors.push(Door {
                number: count,
                prize: false,
            });
        }
        count += 1;
    }
    return doors;
}

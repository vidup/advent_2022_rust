#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum OutCome {
    Win,
    Draw,
    Lose
}

#[derive(Debug)]
struct Round {
    player_1: Shape,
    player_2: Shape,
}

// i'm player 2
fn get_outcome_from_round(round: &Round) -> Option<OutCome> {
    match round.player_2 {
        Shape::Rock => match round.player_1 {
            Shape::Rock => Some(OutCome::Draw),
            Shape::Paper => Some(OutCome::Lose),
            Shape::Scissors => Some(OutCome::Win),
        },
        Shape::Paper => match round.player_1 {
            Shape::Rock => Some(OutCome::Win),
            Shape::Paper => Some(OutCome::Draw),
            Shape::Scissors => Some(OutCome::Lose),
        },
        Shape::Scissors => match round.player_1 {
            Shape::Rock => Some(OutCome::Lose),
            Shape::Paper => Some(OutCome::Win),
            Shape::Scissors => Some(OutCome::Draw),
        },
    }
}

fn get_shape_value(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_outcome_value(outcome: &OutCome) -> u32 {
    match outcome {
        OutCome::Win => 6,
        OutCome::Draw => 3,
        OutCome::Lose => 0,
    }
}

fn get_round_score(shape: &Shape, outcome: OutCome) -> u32 {
    get_shape_value(shape) + get_outcome_value(&outcome)
}

fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_2/input.txt").expect("Unable to read file");
    contents
}

fn parse_file_1(file_content: String) -> Vec<Round> {
    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut rounds_raw_data: Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect()).collect();
    rounds_raw_data.pop(); // remove last empty line
    
    let rounds_data: Vec<Round> = rounds_raw_data.iter().map(|round| {
        let player_1_shape = match round[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape {}", round[0]),
        };

        let player_2_shape = match round[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid shape {}", round[1]),
        };
        
        Round {
            player_1: player_1_shape,
            player_2: player_2_shape,
        }
    }).collect();

    rounds_data
}


fn parse_file_2(file_content: String) -> Vec<Round> {
    let lines: Vec<&str> = file_content.split("\n").collect();
    let mut rounds_raw_data: Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect()).collect();
    rounds_raw_data.pop(); // remove last empty line
    
    let rounds_data: Vec<Round> = rounds_raw_data.iter().map(|round| {
        let player_1_shape = match round[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid shape {}", round[0]),
        };

        let expected_result = match round[1] {
            "X" => OutCome::Lose,
            "Y" => OutCome::Draw,
            "Z" => OutCome::Win,
            _ => panic!("Invalid shape {}", round[1]),
        };

        let player_2_shape = match expected_result {
            OutCome::Draw => match player_1_shape {
                Shape::Rock => Shape::Rock,
                Shape::Paper => Shape::Paper,
                Shape::Scissors => Shape::Scissors,
            },
            OutCome::Win => match player_1_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            OutCome::Lose => match player_1_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        };
        
        Round {
            player_1: player_1_shape,
            player_2: player_2_shape,
        }
    }).collect();

    rounds_data
}

pub fn part_1() {
    let file_content = read_file();
    let rounds_data = parse_file_1(file_content);
    let rounds_scores: Vec<u32> = rounds_data.iter().map(|round| {
        let outcome = get_outcome_from_round(round).expect("failed_to_get_outcome");
        let score = get_round_score(&round.player_2, outcome);
        score
    }).collect();

    println!("Level 2, Part 1");
    println!("Total score: {}", rounds_scores.iter().sum::<u32>());
}

pub fn part_2() {
    let file_content = read_file();
    let rounds_data = parse_file_2(file_content);
    let rounds_scores: Vec<u32> = rounds_data.iter().map(|round| {
        let outcome = get_outcome_from_round(round).expect("failed_to_get_outcome");
        let score = get_round_score(&round.player_2, outcome);
        score
    }).collect();

    println!("Level 2, Part 2");
    println!("Total score: {}", rounds_scores.iter().sum::<u32>());
}
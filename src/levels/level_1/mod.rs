fn read_input_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_1/input.txt").expect("Unable to read file");
    contents
}

fn group_data(contents: &str) -> Vec<i32> {
    let mut calories_by_elves: Vec<i32> = Vec::new();
    calories_by_elves.push(0);

    for line in contents.lines() {
        let len = calories_by_elves.len();
        if line.is_empty() {
            calories_by_elves.push(0);
            continue;
        }

        let calories = line.parse::<i32>().unwrap();
        calories_by_elves[len - 1] = calories_by_elves[len - 1] + calories;
    }

    return calories_by_elves;
}

fn order_by_decreasing_calorific_potential(calories_by_elves: &Vec<i32>) -> Vec<i32> {
    let mut calories_by_elves = calories_by_elves.clone();
    calories_by_elves.sort();
    calories_by_elves.reverse();
    return calories_by_elves;
}

pub fn part_1() {
    println!("\nDay 1, Part 1");
    let contents = read_input_file();
    let calories_by_elves = group_data(&contents);
    let max_to_min_calories = order_by_decreasing_calorific_potential(&calories_by_elves);

    println!("Most calorific elve: {}", max_to_min_calories[0]);
}

pub fn part_2() {
    println!("\nDay 1, Part 2");
    let contents = read_input_file();
    let calories_by_elves = group_data(&contents);
    let max_to_min_calories = order_by_decreasing_calorific_potential(&calories_by_elves);
    let sum_of_three_most_calorific_elves = max_to_min_calories[0..3].iter().sum::<i32>();

    println!("Sum of three most calorific elves: {}", sum_of_three_most_calorific_elves);
}
use std::fs::read;

pub fn process() {
    let file_content = read_file();
    let rucksacks: Vec<&str> = file_content.lines().collect();

    part_1(&rucksacks);
    part_2(&rucksacks);
}

fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_3/input.txt").expect("Unable to read file");
    contents
}

fn get_item_priority(character: char) -> u32 {
    let value_in_ascii = character as u32;
    let priority = match value_in_ascii {
        97..=122 => value_in_ascii - 96, // a = 97 => 97 - 96 = 1
        65..=90 => value_in_ascii - 38, // A = 65 => 65 - 38 = 27
        0 => 0,
        _ => panic!("Invalid character {}", character),
    };

    priority
}

pub fn part_1(rucksacks: &Vec<&str>) {
    let mut sum_of_priorities_of_common_items = 0;
    for rucksack in rucksacks {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
        for character in compartment_1.chars() {
            if compartment_2.contains(character) {
                sum_of_priorities_of_common_items += get_item_priority(character);
                break;
            }
        }
    }

    println!("Level 1 bis: ");
    println!("Priority sum for common item between containers: {}", sum_of_priorities_of_common_items);
}

pub fn part_2(rucksacks: &Vec<&str>) {
    let mut sum_of_priorities_of_rucksack_groups_common_items = 0;
    for rucksack_group in rucksacks.chunks(3) {
        for character in rucksack_group[0].chars() {
            if rucksack_group[1].contains(character) && rucksack_group[2].contains(character) {
                sum_of_priorities_of_rucksack_groups_common_items += get_item_priority(character);
                break;
            }
        }
    }

    println!("Level 2 bis: ");
    println!("Priority sum for common item between rucksack groups: {}", sum_of_priorities_of_rucksack_groups_common_items);
}
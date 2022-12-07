use std::collections::HashMap;

pub fn process() {
    let file_content = read_file();
    let data = file_content.split(" 1   2   3   4   5   6   7   8   9 ").collect::<Vec<&str>>();
    let mut crates: Vec<&str> = data[0].lines().collect();
    let instructions: Vec<Instruction> = data[1].lines().map(
        |line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            if parts.len() != 6 {
                return Instruction {
                    from: 0,
                    to: 0,
                    amount: 0
                }
            }

            let amount = parts[1].parse::<i32>().unwrap();
            let from = parts[3].parse().unwrap();
            let to = parts[5].parse().unwrap();

            Instruction {
                from,
                to,
                amount
            }
        }
    ).collect();

    
    // create a hashmap with keys 1-9 and values []
    let mut crates_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 1..10 {
        crates_map.insert(i, vec![]);
    }

    crates.reverse();
    for crate_line in crates {
        for i in 0..9 {
            let crate_letter = crate_line.chars().nth((i * 4) + 1).expect("aaaaaa");
            match crate_letter {
                // any letter
                'A'..='Z' => {
                    crates_map.get_mut(&(i as i32 + 1)).unwrap().push(crate_letter as i32);
                },
                _ => {}
            }
        }
    }

    part_1(&mut crates_map.clone(), &instructions); // It will be modified by the script, so I need to clone it to avoid polluting the second part
    part_2(&mut crates_map, &instructions);
}

struct Instruction {
    from: i32,
    to: i32,
    amount: i32,
}

fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_5/input.txt").expect("Unable to read file");
    contents
}

fn move_crates_one_by_one(
    crates: &mut HashMap<i32, Vec<i32>>,
    instruction: &Instruction
) {
    let mut i = 0;
    while i < instruction.amount {
        let from = crates.get_mut(&instruction.from).unwrap();
        let crate_to_move = from.pop().unwrap();

        let to = crates.get_mut(&instruction.to).unwrap();
        to.push(crate_to_move);
        i += 1;
    }
}


fn get_top_shelf(crates: &mut HashMap<i32, Vec<i32>>) -> String {
    let mut result = String::new();
    for i in 1..10 {
        let crate_stack = crates.get(&i).unwrap();
        let crate_letter = crate_stack.last().unwrap();
        result.push(*crate_letter as u8 as char);
    }
    result
}

fn part_1(
    crates: &mut HashMap<i32, Vec<i32>>,
    instructions: &Vec<Instruction>
) {
    for instruction in instructions {
        move_crates_one_by_one(crates, instruction);
    }

    let result = get_top_shelf(crates);

    println!("Level 1 : ");
    println!("Result: {}", result);

}

fn move_crates_in_one_go(
    crates: &mut HashMap<i32, Vec<i32>>,
    instruction: &Instruction
) {
    if instruction.amount == 0 {
        return;
    }

    
    let from = crates.get_mut(&instruction.from).unwrap();
    let crates_to_move = from.split_off(from.len() - instruction.amount as usize);
    
    let to = crates.get_mut(&instruction.to).unwrap();
    // push all at once
    to.extend(crates_to_move);
}

fn part_2(
    crates: &mut HashMap<i32, Vec<i32>>,
    instructions: &Vec<Instruction>
) {
    for instruction in instructions {
        move_crates_in_one_go(crates, instruction);
    }

    let result = get_top_shelf(crates);

    println!("Level 2 : ");
    println!("Result: {}", result);
}

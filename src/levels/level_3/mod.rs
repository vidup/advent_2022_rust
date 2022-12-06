fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_3/input.txt").expect("Unable to read file");
    contents
}

fn split_into_sacks(file_content: &str) -> Vec<Vec<String>> {
    let mut sacks: Vec<Vec<String>> = Vec::new();
    for line in file_content.lines() {
        let mut sack: Vec<String> = Vec::new();
        for item in line.split("") {
            if item != "" {
                sack.push(item.to_string());
            }
        }
        sacks.push(sack);
    }

    sacks
}

struct Container {
    items: Vec<String>
}

struct Sack {
    container_1: Container,
    container_2: Container,
    items: Vec<String>
}


fn split_content_in_two_containers(content: &str) -> (Container, Container) {
    let index_of_split = content.len() / 2; // always even number (spec)
    let container_1_items = content[..index_of_split].chars().map(|c| c.to_string()).collect();
    let container_2_items = content[index_of_split..].chars().map(|c| c.to_string()).collect();
    
    (Container { items: container_1_items }, Container { items: container_2_items })
}

impl Sack {
    fn new(content: String) -> Sack {
        let (container_1, container_2) = split_content_in_two_containers(&content);
        Sack {
            container_1,
            container_2,
            items: content.chars().map(|c| c.to_string()).collect()
        }
    }

    fn find_common_item_in_both_containers(&self) -> String {
        let mut common_item: Option<String> = None;
        for item in &self.container_1.items {
            if self.container_2.items.contains(item) {
                common_item = Some(item.to_string());
                break;
            }
        }

        match common_item {
            Some(item) => item,
            None => panic!("No common item found"),
        }
    }

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

struct SackGroupOfThree {
    sack_1: Sack,
    sack_2: Sack,
    sack_3: Sack,
}

fn find_common_item_between_sacks(sackgroup: &SackGroupOfThree) -> char {
    let sack_1_items = &sackgroup.sack_1.items;
    let sack_2_items = &sackgroup.sack_2.items;
    let sack_3_items = &sackgroup.sack_3.items;

    let mut common_item: Option<char> = None;
    for item in sack_1_items {
        if sack_2_items.contains(&item) && sack_3_items.contains(&item) {
            common_item = Some(item.chars().next().unwrap());
            break;
        }
    }

    match common_item {
        Some(item) => item,
        None => panic!("No common item found"),
    }
}

impl SackGroupOfThree {
    fn new(sack_1: Sack, sack_2: Sack, sack_3: Sack) -> SackGroupOfThree {
        SackGroupOfThree {
            sack_1,
            sack_2,
            sack_3,
        }
    }    
}

pub fn part_1() {
    let file_content = read_file();
    let sacks_raw_data = split_into_sacks(&file_content);

    // same functional
    let sacks: Vec<Sack> = sacks_raw_data
        .iter()
        .map(|sack_raw_data| Sack::new(sack_raw_data.join("")))
        .collect();
    
    let sum_of_priorities_of_common_items = sacks
        .iter()
        .map(|sack| sack.find_common_item_in_both_containers())
        .map(|common_item| get_item_priority(common_item.chars().next().unwrap()))
        .sum::<u32>();

    println!("Level 1: ");
    println!("Sacks count: {}", sacks.len());
    println!("Sum of priorities of common items: {}", sum_of_priorities_of_common_items);

    // split sacks in groups of
    let mut sacks_groups: Vec<Vec<Sack>> = Vec::new();
    let mut sack_group: Vec<Sack> = Vec::new();
    for sack in sacks {
        sack_group.push(sack);
        if sack_group.len() == 3 {
            sack_group.clear();
        }
    }

    let groups_of_three = sacks_groups.iter().map(|group| {
        let group_of_three = SackGroupOfThree::new(group[0], group[1], group[2]);
        group_of_three
    }).collect::<Vec<SackGroupOfThree>>();
    
    

    let sum_of_priorities_of_common_items_in_groups_of_three = groups_of_three
        .iter()
        .map(|sack_group_of_three| find_common_item_between_sacks(&sack_group_of_three))
        .map(|common_item| get_item_priority(common_item))
        .sum::<u32>();

    println!("Level 2: ");
    println!("Sum of priorities of common items in groups of three: {}", sum_of_priorities_of_common_items_in_groups_of_three);

}
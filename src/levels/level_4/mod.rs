pub fn process() {
    let file_content = read_file();
    let assignments: Vec<SectionAssignment> = file_content.lines().collect::<Vec<&str>>()
        .iter()
        .map(|line| {
            let pairs = line.split(",").collect::<Vec<&str>>();
            let pair_one = pairs[0].split("-").collect::<Vec<&str>>();
            let pair_two = pairs[1].split("-").collect::<Vec<&str>>();

            SectionAssignment {
                pair_one_start: pair_one[0].parse::<usize>().unwrap(),
                pair_one_end: pair_one[1].parse::<usize>().unwrap(),

                pair_two_start: pair_two[0].parse::<usize>().unwrap(),
                pair_two_end: pair_two[1].parse::<usize>().unwrap(),
            }
        }).collect();

    part_1(&assignments);
    part_2(&assignments);
}

struct SectionAssignment {
    pair_one_start: usize,
    pair_one_end: usize,

    pair_two_start: usize,
    pair_two_end: usize,
}

fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_4/input.txt").expect("Unable to read file");
    contents
}

fn pair_one_is_inside_pair_2(a: &SectionAssignment) -> bool {
    a.pair_one_start >= a.pair_two_start && a.pair_one_end <= a.pair_two_end
}
fn pair_two_is_inside_pair_1(a: &SectionAssignment) -> bool {
    a.pair_two_start >= a.pair_one_start && a.pair_two_end <= a.pair_one_end
}

fn part_1(assignments: &Vec<SectionAssignment>) {
    let mut overlaps = 0;
    for a in assignments {
        if pair_one_is_inside_pair_2(a) || pair_two_is_inside_pair_1(a) {
            overlaps += 1;
        }
    }

    println!("Level 1: ");
    println!("Overlaps: {}", overlaps);
}

fn pair_one_starts_in_pair_2(a: &SectionAssignment) -> bool {
    a.pair_one_start >= a.pair_two_start && a.pair_one_start <= a.pair_two_end
}

fn pair_2_starts_in_pair_1(a: &SectionAssignment) -> bool {
    a.pair_two_start >= a.pair_one_start && a.pair_two_start <= a.pair_one_end
}

fn part_2(assignments: &Vec<SectionAssignment>) {
    let mut overlaps = 0;
    for a in assignments {
        if pair_one_starts_in_pair_2(a) || pair_2_starts_in_pair_1(a) {
            overlaps += 1;
        }
    }
    
    println!("Level 2: ");
    println!("Overlaps: {}", overlaps);
}
pub mod file {
    pub fn read(level: i32) -> String {
        let real_path = String::from("./src/levels/level_") + &level.to_string() + "/input.txt";
        let content = std::fs::read_to_string(real_path).expect("Something went wrong reading the file");
        content
    }
}

pub fn introduce_level(level: i32, part: i32) {
    println!("\nLevel {}", level);
    println!("Part {}", part);
}
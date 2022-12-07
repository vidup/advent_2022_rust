pub fn process() {
    let file_content = read_file();
    let COMMUNICATION_MARKER_LENGTH = 4;
    let MESSAGES_MARKER_LENGTH = 14;
    
    let first_marker_index = identifyContent(&file_content, COMMUNICATION_MARKER_LENGTH);

    println!("Level 6 - part 1");
    println!("First marker index: {}", first_marker_index);
    println!("End of first marker index: {}", first_marker_index + 4); // expected response counts from 1, we count from 0 => +4 instead of +3

    let first_message_index = identifyContent(&file_content, MESSAGES_MARKER_LENGTH);

    println!("Level 6 - part 2");
    println!("First message index: {}", first_message_index);
    println!("End of first message index: {}", first_message_index + 14); // expected response counts from 1, we count from 0 => +14 instead of +13
}

fn identifyContent(file_content: &str, marker_length: usize) -> i32 {
    let mut first_marker_index = -1;
    let mut index: i32  = 0;
    while first_marker_index == -1 {
        let four_digits_market = file_content.chars().skip(index as usize).take(marker_length).collect::<String>();
        let mut digits = four_digits_market.chars().collect::<Vec<char>>();

        // dedup only remove consecutive duplicates so I sort first to make them adjacent
        digits.sort_unstable();
        digits.dedup();

        if digits.len() != marker_length {
            index += 1;
        } else {
            first_marker_index = index;
        }        
    }
    first_marker_index
}

fn read_file() -> String {
    let contents = std::fs::read_to_string("./src/levels/level_6/input.txt").expect("Unable to read file");
    contents
}

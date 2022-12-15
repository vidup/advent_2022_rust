use std::collections::HashMap;

enum CommandKind {
    CD,
    LS,
}

struct Command {
    content: String,
    kind: CommandKind,
}

struct File {
    path: String,
    size: i32,
}

struct Folder {
    path: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

pub fn process() {
    advent_2022::introduce_level(7, 1);

    let file_content = advent_2022::file::read(7);
    println!("File content: {}", file_content);

    let mut folders: HashMap<String, Folder> = HashMap::new();
    let mut files: HashMap<String, File> = HashMap::new();

    // let commands: Vec<Command> = Vec::new();

    for (index, line) in file_content.lines().enumerate() {
        let is_command = line.starts_with("$");
        if is_command {
            // get all lines after this command
            let all_lines_after_command = &file_content.lines().collect::<Vec<&str>>()[index..];
            let result = get_all_lines_until_next_command(
                all_lines_after_command
            );

            println!("Command: {}", line);
            println!("Result: {:?}", result);
        }
    }
}

fn get_all_lines_until_next_command(lines: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        println!("Line: {}", line);
        if line.starts_with("$") {
            break;
        }
        result.push(line.to_string());
    }
    result
}

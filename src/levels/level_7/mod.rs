use std::collections::HashMap;
// I would have prefered using a tree structure, but I don't know how to do it in Rust.
// Seems like it's really hard to pull off so I went for a loop and a hashmaps instead.
// I'm not sure if it's the best solution, but it works.
// To be honest I'm sure it's a shit solution haha, it might contain a bug or two but 
// it passed the advent tests. Might be worth it to come back to it later and try to
// improve it. I put .clone() everywhere because I'm not confortable with Rust's ownership
// system yet. One day I'll get it, but not today.

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
    let file_content = advent_2022::file::read(7);
    println!("File content: {}", file_content);

    let mut folders: HashMap<String, Folder> = HashMap::new();
    folders.insert(String::from("/"), Folder {
        path: String::from("/"),
        files: Vec::new(),
        folders: Vec::new(),
    });
    let mut files: HashMap<String, File> = HashMap::new();
    let mut current_directory = String::from("/");

    // let commands: Vec<Command> = Vec::new();

    for (index, line) in file_content.lines().enumerate() {
        let is_command = line.starts_with("$");
        if is_command {
            if line.starts_with("$ cd") {
                let new_current_directory = handle_cd_command(line, current_directory.as_str());

                if !folders.contains_key(new_current_directory.as_str()) {
                    let key = new_current_directory.clone();
                    
                    folders.insert(
                        key,
                        Folder {
                            path: String::from(&new_current_directory),
                            files: Vec::new(),
                            folders: Vec::new(),
                        }
                    );
                }

                current_directory = new_current_directory;
                continue;
            } else if line.starts_with("$ ls") {
                let folder_files = get_all_lines_until_next_command(&file_content.lines().skip(index + 1).collect::<Vec<&str>>());
                for file in folder_files {
                    let file_path = String::from(&current_directory) + "/" + &file.path.clone();
                    files.insert(
                        file_path.clone(),
                        File {
                            path: file_path,
                            size: file.size,
                        }
                    );
                }
                continue;
            }

        }
    }

    advent_2022::introduce_level(7, 1);
    let big_folders_min_size = 100000;
    let mut total_size_of_small_folders = 0;

    let mut folders_size: HashMap<String, i32> = HashMap::new();

    for folder in folders {
        let mut folder_size = 0;
        let folder_path = folder.0;

        let folder_files = files.values().filter(
            |file| file.path.starts_with(folder_path.as_str())
            && file.path.split(&folder_path).collect::<Vec<&str>>().len() == 2 // only files in the current folder.
            // if we ignore the second condition, we will get the size of all the files in the subfolders too.
        );

        for file in folder_files {
            folder_size += file.size;
        }

        folders_size.insert(folder_path, folder_size);

        if folder_size < big_folders_min_size {
            total_size_of_small_folders += folder_size;
        }
    }

    println!("Total size of small folders: {}", total_size_of_small_folders);


    advent_2022::introduce_level(7, 2);
    let total_size = 70000000;
    let total_files_size = files.values().fold(0, |acc, file| acc + file.size);
    println!("Total files size: {}", total_files_size);
    let unused_space = total_size - total_files_size;
    println!("Unused space: {}", unused_space);

    let required_unused_space = 30000000;
    let space_to_free = required_unused_space - unused_space;
    println!("Space to free: {} - {} = {}", required_unused_space, unused_space, space_to_free);

    let mut folder_to_delete = File {
        path: String::from("/"),
        size: 999999999,
    };


    for (folder_path, folder_size) in folders_size.iter() {
        if folder_size.is_positive() {
            if folder_size < &folder_to_delete.size && folder_size > &space_to_free {
                folder_to_delete = File {
                    path: folder_path.clone(),
                    size: folder_size.clone(),
                };
            }
        }
    }

    println!("Folder to delete: {}", folder_to_delete.path);
    println!("Folder size: {}", folder_to_delete.size);
    
}

fn handle_cd_command(line: &str, current_directory: &str) -> String {
    println!("CD command: {}", line);
    // ex: $ cd wqqfszn
    // get the path
    let path = line.split(" ").collect::<Vec<&str>>()[2];
    match path {
        "/" => {
            String::from("/")
        },
        ".." => {
            let mut path = String::from(current_directory);
            if path == "/" {
                return String::from("/");
            }

            let mut path_parts = path.split("/").collect::<Vec<&str>>();
            let len = path_parts.len();
            path_parts.remove(path_parts.len() - 1);

            String::from(path_parts.join("/"))
        },
        _ => {
            if current_directory == "/" {
                return String::from("/") + path;
            }
            String::from(current_directory) + "/" + path
        }
    }
}

fn get_all_lines_until_next_command(lines: &[&str]) -> Vec<File>  {
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        println!("Line: {}", line);
        if line.starts_with("$") {
            break;
        }
        result.push(line.to_string());
    }

    // let mut folders: Vec<Folder> = vec![];
    let mut files: Vec<File> = vec![];

    for line in result {
        if line.starts_with("dir") {
            continue;
        } else {
            let size = line.split(" ").collect::<Vec<&str>>()[0];
            let name = line.split(" ").collect::<Vec<&str>>()[1];
            files.push(File { path: String::from(name), size: size.parse::<i32>().unwrap() })
        }
    }

    files
}

use std::env::{args, var};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

fn main() {
    let arguments: Vec<String> = args().collect();
    match arguments.as_slice() {
        [_] => handle_list_argument(),
        [_, arg] if arg == "-l" || arg == "--list" => handle_list_argument(),
        [_, arg, needle] if arg == "-s" || arg == "--search" => handle_search_exact_command(needle),
        [_, arg] if arg == "-h" || arg == "--help" => print_help(),
        _ => print_help(),
    }
}

fn handle_list_argument() {
    let filepath = get_file_path();
    if let Some(contents) = read_file(&filepath) {
        let lines: Vec<&str> = contents.lines().collect();
        println!("AVAILABLE LINUX COMMANDS:");
        for line in lines {
            print_contents(&line);
        }
    } else {
        exit_with_error("Failed to read commands from file.");
    }
}

fn get_file_path() -> String {
    let home_dir = var("HOME").unwrap_or_else(|_| String::from("."));
    let mut path = PathBuf::from(home_dir);
    path.push("commands/linux");
    path.to_str().unwrap().to_string()
}

fn read_file(filepath: &str) -> Option<String> {
    match File::open(filepath) {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                exit_with_error(&format!("Failed to read the file: {}", e));
                return None; // This line won't be reached, but is needed to satisfy the return type.
            }
            Some(contents)
        }
        Err(e) => {
            exit_with_error(&format!("Error opening file: {}", e));
            None
        }
    }
}

fn handle_search_exact_command(needle: &str) {
    let filepath = get_file_path();
    if let Some(haystack) = read_file(&filepath) {
        let found: Vec<&str> = haystack
            .lines()
            .filter(|line| line.to_lowercase().contains(&needle.to_lowercase()))
            .collect();
        if found.is_empty() {
            println!("NO COMMAND FOUND");
            return;
        }
        let total = found.len();
        println!(
            "{} MATCHING COMMAND{}",
            total,
            if total == 1 { "" } else { "S" }
        );
        println!("(*) MEANS THE COMMAND RUN BY ROOT USER");
        for line in found {
            print_contents(line);
        }
    }
}

fn print_help() {
    draw_commands_ascii();
    println!("Commands: Display and Search through linux commandline commands");
    println!("Usage: [-l] [-s <args>] [-h]");
    println!("-l , --list\n\tList all available commands");
    println!("-s , --search <arg>\n\tSearch for argument in commands");
    println!("-h , --help\n\tDisplay this help message");
}

fn exit_with_error(e: &str) {
    eprintln!("ERROR: \n{}", e);
    exit(1);
}

fn print_contents(contents: &str) {
    println!("{}", contents);
}

fn draw_commands_ascii() {
    let text = "
 ██████╗ ██████╗ ███╗   ███╗███╗   ███╗ █████╗ ███╗   ██╗██████╗ ███████╗
██╔════╝██╔═══██╗████╗ ████║████╗ ████║██╔══██╗████╗  ██║██╔══██╗██╔════╝
██║     ██║   ██║██╔████╔██║██╔████╔██║███████║██╔██╗ ██║██║  ██║███████╗
██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██╔══██║██║╚██╗██║██║  ██║╚════██║
╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║  ██║██║ ╚████║██████╔╝███████║
 ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝
                                                                         
";
    println!("{}", text);
}

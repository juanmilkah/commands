use clap::Parser;
use std::env::var;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version,long_about=None, about)]
struct Args {
    #[arg(short = 'l', long = "list")]
    list: Option<String>,

    #[arg(short = 's', long = "search")]
    search: Option<String>,

    #[arg(short = 'i', long = "ignorecase")]
    ignorecase: bool,

    #[arg(short = 'a', long = "ascii")]
    ascii: bool,
}

enum Ignorecase {
    True,
    False,
}

fn main() {
    let arguments = Args::parse();

    if arguments.ascii {
        draw_commands_ascii();
    }

    if let Some(val) = arguments.search {
        if arguments.ignorecase {
            handle_search_exact_command(&val, Ignorecase::True);
            exit(0);
        } else {
            handle_search_exact_command(&val, Ignorecase::False);
            exit(0);
        }
    }

    handle_list_argument();
}

fn handle_list_argument() {
    let filepath = get_file_path();
    if let Some(contents) = read_file(&filepath) {
        let lines: Vec<&str> = contents.lines().collect();
        println!("AVAILABLE LINUX COMMANDS:");
        for line in lines {
            print_contents(line);
        }
    } else {
        exit_with_error("Failed to read commands from file.");
    }
}

fn get_file_path() -> String {
    let home_dir = var("HOME").unwrap_or_else(|_| String::from("."));
    let mut path = PathBuf::from(home_dir);
    path.push(".commands/linux");
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

fn handle_search_exact_command(needle: &str, ignorecase: Ignorecase) {
    let filepath = get_file_path();
    if let Some(haystack) = read_file(&filepath) {
        let found: Vec<&str> = match ignorecase {
            Ignorecase::True => haystack
                .lines()
                .filter(|line| line.to_lowercase().contains(&needle.to_lowercase()))
                .collect(),
            Ignorecase::False => haystack
                .lines()
                .filter(|line| line.contains(needle))
                .collect(),
        };

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

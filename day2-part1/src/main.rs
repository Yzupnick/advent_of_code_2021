use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let (distance, depth) = 
        io::stdin().lock().lines()
                          .map(readline_or_panic)
                          .map(parse_or_panic)
                          .fold((0,0), track_position);
    println!("{} {} {}",distance, depth, distance * depth);
}

enum Command {
    Forward (i32),
    Down (i32),
    Up (i32),
}

fn track_position (state: (i32, i32), next:Command) -> (i32, i32) {
    let (distance, depth) = state;
    match next {
        Command::Forward(i) => { 
            return (distance + i, depth)
        }
        Command::Down(i) => { 
            return (distance, depth + i)
        }
        Command::Up(i) => { 
            return (distance, depth - i)
        }
    }
}


fn parse_or_panic(input: String) -> Command {
    let re = Regex::new(r"^(forward|down|up) (\d+)$").unwrap();
    let found = re.captures(&input);
    match found {
        Some(c) => {
            match c[2].parse::<i32>() {
                Err(_) => {
                    println!("Error parsing line {}", input);
                    std::process::exit(1);
                }
                Ok(i) => {
                    if &c[1] == "forward" {
                        return Command::Forward(i);
                    } else if &c[1] == "down" {
                        return Command::Down(i);
                    } else if &c[1] == "up" {
                        return Command::Up(i);
                    } else {
                        println!("Error parsing line {}", input);
                        std::process::exit(1);
                    }
                }
            }
        }
        None => {
            println!("Error parsing line {}", input);
            std::process::exit(1);
        }
    }
}

fn readline_or_panic(line:Result<String, std::io::Error>) -> String {
    match line {
        Ok(input) => {
            input
        }
        Err(_) => {
            println!("Error reading from stdin");
            std::process::exit(1);
        }
    }
}

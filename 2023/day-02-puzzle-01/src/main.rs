use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

fn main() {
    let f = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let mut total: i32 = 0;

    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();

        let game = match parse_line(&line) {
            Ok(game) => game,
            Err(error) => panic!("There was a problem parsing the line: {:?}", error),
        };

        if !is_valid_game(&game) {
            continue;
        }

        total += game.id;
    }

    println!("Total: {}", total);
}

pub fn is_valid_game(game: &Game) -> bool {
    for set in &game.sets {
        for set_part in &set.set_parts {
            if set_part.color == Color::Red && set_part.count > 12 {
                return false;
            } else if set_part.color == Color::Green && set_part.count > 13 {
                return false;
            } else if set_part.color == Color::Blue && set_part.count > 14 {
                return false;
            }
        }
    }

    true
}

pub fn parse_line(line: &str) -> Result<Game, &'static str> {
    println!("Parsing line: {}", line);
    let parts: Vec<&str> = line.split(":").collect();
    if parts.len() != 2 {
        return Err("Invalid line");
    }

    let mut game = Game {
        id: parts[0]
            .trim()
            .trim_start_matches("Game ")
            .parse::<i32>()
            .map_err(|_| "Invalid ID")?,
        sets: Vec::new(),
    };

    let mut set_parts_vec = Vec::new();
    let sets: Vec<&str> = parts[1].split(",").collect();
    for mut set in sets {
        set = set.trim();
        let set_parts: Vec<&str> = set.split(";").collect();

        for mut set_part in set_parts {
            set_part = set_part.trim();
            let set_part_parts: Vec<&str> = set_part.split(" ").collect();

            if set_part_parts.len() != 2 {
                return Err("Invalid line");
            }

            let count = set_part_parts[0]
                .parse::<i32>()
                .map_err(|_| "Invalid count")?;

            let color = match set_part_parts[1] {
                "blue" => Color::Blue,
                "red" => Color::Red,
                "green" => Color::Green,
                _ => return Err("Invalid line"),
            };

            set_parts_vec.push(SetParts { count, color });
        }
    }
    game.sets.push(Sets {
        set_parts: set_parts_vec.clone(),
    });

    Ok(game)
}

#[derive(Debug)]
pub struct Game {
    id: i32,
    sets: Vec<Sets>,
}

#[derive(Debug)]
pub struct Sets {
    set_parts: Vec<SetParts>,
}

#[derive(Debug, Clone)]
pub struct SetParts {
    count: i32,
    color: Color,
}

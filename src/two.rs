use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

#[allow(dead_code)]
impl Draw {
    fn new(colors: Vec<u8>) -> Self {
        Self {
            red: colors[0],
            green: colors[1],
            blue: colors[2],
        }
    }
}

#[allow(dead_code)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[allow(dead_code)]
impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            draws: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub fn part_one(input: File) -> u64 {
    let mut sum: u64 = 0;

    let reader = BufReader::new(input);

    let game_colors: Vec<u8> = vec![12, 13, 14];
    let game_config: Draw = Draw::new(game_colors);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parsed_line: Vec<String> = line
            .split(|c| c == ';' || c == ':')
            .map(|s| s.to_string())
            .collect();
        println!("parsed_line is : {:?}", parsed_line);
        let game_id: u32 = parsed_line[0]
            .chars()
            .filter(|&c| c.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        println!("game_id is : {}", game_id);

        let current_elf_game: Game = Game::new(game_id);
        sum += current_elf_game.id as u64;

        parsed_line.remove(0);
        let mut draws: Vec<Vec<String>> = Vec::new();
        for draw in parsed_line.iter() {
            draws.push(draw.split(|c| c == ',').map(|s| s.to_string()).collect());
        }
        println!("draws is : {:?}", draws);
        for draw in draws.iter() {
            let mut colors: Vec<u8> = vec![0; 3];
            for color in draw.iter() {
                let num: u8 = color
                    .chars()
                    .filter(|&c| c.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let color_word: String = color.chars().filter(|&c| c.is_alphabetic()).collect();
                match color_word.as_str() {
                    "red" => colors[0] = num,
                    "green" => colors[1] = num,
                    _ => colors[2] = num,
                }
            }
            println!("colors were: {:?}", colors);
            let last_draw: Draw = Draw::new(colors);
            if game_config.red < last_draw.red
                || game_config.green < last_draw.green
                || game_config.blue < last_draw.blue
            {
                println!("game {} is not possible", current_elf_game.id);
                sum -= current_elf_game.id as u64;
                break;
            }
        }
        println!("sum was: {}", sum);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let file: File = File::open("./src/test/two.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_two() {
        let file: File = File::open("./src/real/two.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 8);
    }
}

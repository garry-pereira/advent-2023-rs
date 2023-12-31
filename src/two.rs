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

fn remove_colon_semicolons(line: &str) -> Vec<String> {
    line.split(|c| c == ';' || c == ':')
        .map(|s| s.to_string())
        .collect()
}

fn get_game_id(line: &str) -> u32 {
    line.chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn get_color_number(color: &str) -> u8 {
    color
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn get_color_word(color: &str) -> String {
    color.chars().filter(|&c| c.is_alphabetic()).collect()
}

#[allow(dead_code)]
pub fn part_one(input: File) -> u64 {
    let mut sum: u64 = 0;

    let reader = BufReader::new(input);

    let game_colors: Vec<u8> = vec![12, 13, 14];
    let game_config: Draw = Draw::new(game_colors);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut parsed_line: Vec<String> = remove_colon_semicolons(&line);
        println!("parsed_line is : {:?}", parsed_line);

        let game_id: u32 = get_game_id(&parsed_line[0]);
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
                let num: u8 = get_color_number(&color);
                let color_word: String = get_color_word(&color);
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

#[allow(dead_code)]
pub fn part_two(input: File) -> u64 {
    let mut sum: u64 = 0;

    let reader = BufReader::new(input);

    // for each game, we are going to look at each draw and then store the highest
    for line in reader.lines() {
        let line = line.unwrap();
        let mut game_max: Draw = Draw::new(vec![0, 0, 0]);

        let mut parsed_line: Vec<String> = remove_colon_semicolons(&line);
        println!("parsed_line is : {:?}", parsed_line);

        // let game_id: u32 = get_game_id(&parsed_line[0]);
        // println!("game_id is : {}", game_id);

        // let current_elf_game: Game = Game::new(game_id);
        // sum += current_elf_game.id as u64;
        parsed_line.remove(0);
        let mut draws: Vec<Vec<String>> = Vec::new();

        for draw in parsed_line.iter() {
            draws.push(draw.split(|c| c == ',').map(|s| s.to_string()).collect());
        }

        println!("draws is : {:?}", draws);

        for draw in draws.iter() {
            let mut colors: Vec<u8> = vec![0; 3];

            for color in draw.iter() {
                let num: u8 = get_color_number(&color);
                let color_word: String = get_color_word(&color);
                match color_word.as_str() {
                    "red" => colors[0] = num,
                    "green" => colors[1] = num,
                    _ => colors[2] = num,
                }
            }

            println!("colors were: {:?}", colors);
            let last_draw: Draw = Draw::new(colors);

            if last_draw.red > game_max.red {
                game_max.red = last_draw.red;
            }

            if last_draw.green > game_max.green {
                game_max.green = last_draw.green;
            }

            if last_draw.blue > game_max.blue {
                game_max.blue = last_draw.blue;
            }
        }
        sum += game_max.red as u64 * game_max.green as u64 * game_max.blue as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_one() {
    //     let file: File = File::open("./src/test/two.txt").unwrap();
    //     let output: u64 = part_one(file);
    //     println!("answer to part one test: {}", output);
    //     assert_eq!(output, 8);
    // }

    // #[test]
    // fn real_one() {
    //     let file: File = File::open("./src/real/two.txt").unwrap();
    //     let output: u64 = part_one(file);
    //     println!("answer to part one test: {}", output);
    //     assert_eq!(output, 8);
    // }

    // #[test]
    // fn test_two() {
    //     let file: File = File::open("./src/test/two.txt").unwrap();
    //     let output: u64 = part_two(file);
    //     println!("answer to part one test: {}", output);
    //     assert_eq!(output, 2286);
    // }

    #[test]
    fn real_one() {
        let file: File = File::open("./src/real/two.txt").unwrap();
        let output: u64 = part_two(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 8);
    }
}

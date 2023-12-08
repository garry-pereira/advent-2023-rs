use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code, unused_variables)]
pub fn part_one(input: File) -> u64 {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let sum: u64 = 0;
    let mut digits_in_grid: Vec<(usize, usize)> = Vec::new();
    let mut symbols_in_grid: Vec<(usize, usize)> = Vec::new();
    let hotspot: Vec<(usize, usize)> = Vec::new();

    let reader = BufReader::new(input);

    for (r, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        for (c, letter) in line.chars().enumerate() {
            if letter.is_numeric() {
                digits_in_grid.push((r, c));
            }
            if !letter.is_numeric() && letter != '.' {
                symbols_in_grid.push((r, c));
            }
            row.push(letter);
        }
        grid.push(row);
    }
    println!("{:?}", grid);
    println!("{:?}", digits_in_grid);
    println!("{:?}", symbols_in_grid);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let file: File = File::open("./src/test/three.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 142);
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn word_to_number(word: &str) -> Option<u64> {
    let number_words = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    number_words.iter().find_map(
        |(key, &num)| {
            if word.contains(key) {
                Some(num)
            } else {
                None
            }
        },
    )
}

fn first_num(line: &str) -> (u64, isize) {
    let (mut num, mut index) = (0, -1);
    for (i, c) in line.chars().enumerate() {
        if let Some(n) = c.to_digit(10) {
            num = (n as u64) * 10;
            index = i as isize;
            break;
        }
    }
    (num, index)
}

fn last_num(line: &str) -> (u64, usize) {
    let (mut num, mut index) = (0, 0);
    let mut right = line.chars().count() - 1;
    for _ in line.chars() {
        if let Some(n) = line.chars().nth(right).unwrap().to_digit(10) {
            num = n as u64;
            index = right;
            break;
        } else if right > 0 {
            right -= 1;
        } else {
            break;
        }
    }
    (num, index)
}

#[allow(dead_code)]
fn first_word(line: &str, slice: isize) -> Option<u64> {
    let mut num: Option<u64> = None;
    let l_slice: &str = if slice == -1 {
        &line
    } else {
        &line[0..slice as usize]
    };

    let mut l_check: String = String::new();

    for char in l_slice.chars() {
        l_check.push(char);
        if let Some(n) = word_to_number(&l_check) {
            num = Some((n as u64) * 10);
            break;
        }
    }
    num
}

#[allow(dead_code)]
fn last_word(line: &str, slice: usize) -> Option<u64> {
    let mut num: Option<u64> = None;
    let slice_len = line.chars().count();
    let r_slice: &str = &line[slice + 1..slice_len];
    let mut r_check: String = String::new();

    for c in r_slice.chars().rev() {
        r_check.insert(0, c);
        if let Some(n) = word_to_number(&r_check) {
            num = Some(n as u64);
            break;
        }
    }
    num
}

#[allow(dead_code)]
pub fn part_one(input: File) -> u64 {
    let mut sum: u64 = 0;

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();
        let (mut f_num, f_num_index) = first_num(line.as_ref());
        let (mut l_num, l_num_index) = last_num(line.as_ref());
        if let Some(n) = first_word(line.as_ref(), f_num_index) {
            f_num = n;
        }
        if let Some(n) = last_word(line.as_ref(), l_num_index) {
            l_num = n;
        }
        println!("{} and {}", f_num, l_num);
        sum += f_num + l_num;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let file: File = File::open("./src/test/one.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 142);
    }

    #[test]
    fn real_one() {
        let file: File = File::open("./src/real/one.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one real: {}", output);
        assert_eq!(output, 54877);
    }

    #[test]
    fn test_two() {
        let file: File = File::open("./src/test/one_two.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part two test: {}", output);
        assert_eq!(output, 281);
    }
}

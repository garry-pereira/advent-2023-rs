use std::fs::File;
use std::io::{BufRead, BufReader};

fn word_to_number(word: &str) -> Option<u8> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[allow(dead_code)]
pub fn part_one(input: File) -> u64 {
    let mut sum: u64 = 0;

    let reader = BufReader::new(input);

    for line in reader.lines() {
        // indexes
        let (mut left, mut right, line_len) = (
            0 as usize,
            line.as_ref().expect("no line").chars().count() - 1,
            line.as_ref().expect("no line").chars().count(),
        );
        // switch flips on first occurence of number
        let (mut r_check, mut l_check, mut no_num) = (true, true, false);
        let (mut r_first_word, mut l_first_word) = (false, false);
        // holds numbers when numbers are met, does not add them to sum yet
        let (mut l_hold, mut r_hold) = (0, 0);
        for char in line.as_ref().unwrap().chars() {
            if l_check {
                if let Some(n) = char.to_digit(10) {
                    l_hold += (n as u64) * 10;
                    l_check = false;
                } else {
                    left += 1;
                }
            }

            if r_check {
                if let Some(n) = line
                    .as_ref()
                    .unwrap()
                    .chars()
                    .nth(right)
                    .unwrap()
                    .to_digit(10)
                {
                    r_hold += n as u64;
                    r_check = false;
                } else {
                    right -= 1;
                }
            }

            if left == line_len && !no_num {
                no_num = true;
            }

            if (!l_check && !r_check) || no_num {
                // // iterate over the left slice
                // let l_slice: &str = &line.as_ref().unwrap()[0..left];
                // let mut l_check: String = String::new();

                // for char in l_slice.chars() {
                //     if char.is_alphabetic() {
                //         l_check.push(char);
                //         if let Some(n) = word_to_number(&l_check) {
                //             sum += n as u64 * 10;
                //             l_hold = 0;
                //             break;
                //         }
                //     } else {
                //         l_check.clear();
                //     }
                // }

                // // iterate over right slice
                // let r_slice: &str = &line.as_ref().unwrap()[(right + 1)..line_len];
                // let mut r_check: String = String::new();

                // for char in r_slice.chars() {
                //     if char.is_alphabetic() {
                //         r_check.push(char);
                //         if let Some(n) = word_to_number(&r_check) {
                //             sum += n as u64;
                //             r_hold = 0;
                //             break;
                //         }
                //     } else {
                //         r_check.clear();
                //     }
                // }

                sum += l_hold;
                sum += r_hold;
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        // test input and expect_out come from here
        let file: File = File::open("./src/test/one.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 142);
    }

    #[test]
    fn real_one() {
        // test input and expect_out come from here
        let file: File = File::open("./src/real/one.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one real: {}", output);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_two() {
        let file: File = File::open("./src/test/one_two.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part two test: {}", output);
        assert_eq!(output, 281);
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
struct Numbers {
    number: u32,
    occupation: Vec<(usize, usize)>,
}

impl Numbers {
    fn new(number: u32, occupation: Vec<(usize, usize)>) -> Self {
        Self { number, occupation }
    }
}

fn hotspot_gen(
    sym_index: (usize, usize),
    block_dim: usize,
    curr_line_len: usize,
) -> Vec<(usize, usize)> {
    let mut hotspot: Vec<(usize, usize)> = Vec::new();
    let mut x_mover: Vec<isize> = vec![0];
    let mut y_mover: Vec<isize> = vec![0];
    if sym_index.0 != 0 {
        y_mover.push(-1);
    }
    if sym_index.1 != block_dim - 1 {
        y_mover.push(1);
    }
    if sym_index.1 != 0 {
        x_mover.push(-1);
    }
    if sym_index.1 != curr_line_len - 1 {
        x_mover.push(1);
    }
    for (_, r) in y_mover.iter().enumerate() {
        for (_, c) in x_mover.iter().enumerate() {
            if !(r == &0 && c == &0) {
                hotspot.push((
                    (sym_index.0 as isize + r) as usize,
                    (sym_index.1 as isize + c) as usize,
                ));
            }
        }
    }
    hotspot
}

#[allow(dead_code)]
pub fn part_one(input: File) -> u64 {
    let mut sum: u64 = 0;
    let mut hotspot: Vec<(usize, usize)> = Vec::new();
    let mut maybe_numbers: Vec<Numbers> = Vec::new();

    let reader = BufReader::new(&input);
    let block_len = 140;

    for (r, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line_len = line.len();
        let mut holder: String = String::new();
        let mut coord_holder: Vec<(usize, usize)> = Vec::new();
        for (c, letter) in line.chars().enumerate() {
            if letter.is_numeric() {
                holder.push(letter);
                coord_holder.push((r, c));
            } else {
                if holder.len() != 0 {
                    maybe_numbers.push(Numbers::new(
                        holder.parse::<u32>().unwrap(),
                        coord_holder.clone(),
                    ));
                }
                holder.clear();
                coord_holder.clear();
            }
            if !letter.is_numeric() && letter != '.' {
                coord_holder = hotspot_gen((r, c), block_len, line_len);
                hotspot.append(&mut coord_holder);
                coord_holder.clear();
            }
        }
    }

    // iterate through numbers and find is occupation is a hotspot, if it is, then add number to sum
    for current_number in maybe_numbers {
        for occupation in current_number.occupation {
            if hotspot.contains(&occupation) {
                sum += current_number.number as u64;
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
        let file: File = File::open("./src/test/three.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 4361);
    }

    #[test]
    fn real_one() {
        let file: File = File::open("./src/real/three.txt").unwrap();
        let output: u64 = part_one(file);
        println!("answer to part one test: {}", output);
        assert_eq!(output, 530537);
    }
}

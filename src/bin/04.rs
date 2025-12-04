advent_of_code::solution!(4);

const CLEAR: u8 = 0;
const ROLL: u8 = 1;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

pub fn parse_input(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    let real_width = cols + 2;
    let real_height = rows + 2;

    let mut data = vec![CLEAR; real_width * real_height];

    for (y, line) in lines.iter().enumerate() {
        for (x, col) in line.chars().enumerate() {
            if col == '@' {
                data[(y + 1) * real_width + (x + 1)] = ROLL;
            }
        }
    }
    Grid {
        width: real_width,
        height: real_height,
        data,
    }
}

pub fn print_grid(width: usize, height: usize, data: &[u8]) {
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if data[idx] == 1 {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("\n");
    }
}

pub fn count_neighbours(idx: usize, width: usize, data: &[u8]) -> u8 {
    data[idx - width - 1]
        + data[idx - width]
        + data[idx - width + 1]
        + data[idx - 1]
        + data[idx + 1]
        + data[idx + width - 1]
        + data[idx + width]
        + data[idx + width + 1]
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut number_of_acc: u64 = 0;
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let idx = y * grid.width + x;
            if grid.data[idx] == ROLL && count_neighbours(idx, grid.width, &grid.data) < 4 {
                number_of_acc += 1;
            }
        }
    }
    Some(number_of_acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut number_of_removed: u64 = 0;
    let mut queue: Vec<usize> = Vec::new();
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let idx = y * grid.width + x;
            if grid.data[idx] == ROLL && count_neighbours(idx, grid.width, &grid.data) < 4 {
                grid.data[idx] = CLEAR;
                queue.push(idx);
                number_of_removed += 1;
            }
        }
    }
    while let Some(dead_idx) = queue.pop() {
        let w = grid.width;

        let neighbours = [
            dead_idx - w - 1,
            dead_idx - w,
            dead_idx - w + 1,
            dead_idx - 1,
            dead_idx + 1,
            dead_idx + w - 1,
            dead_idx + w,
            dead_idx + w + 1,
        ];
        for &idx in &neighbours {
            if grid.data[idx] == ROLL && count_neighbours(idx, w, &grid.data) < 4 {
                grid.data[idx] = CLEAR;
                queue.push(idx);
                number_of_removed += 1;
            }
        }
    }
    Some(number_of_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

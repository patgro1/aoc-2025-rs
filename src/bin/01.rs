advent_of_code::solution!(1);

#[derive(Debug)]
pub enum Rotation {
    Left(u32),
    Right(u32),
}

pub fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|x| {
            let (dir, count) = x.split_at(1);
            let count: u32 = count.parse().expect("This should not fail");
            match dir {
                "L" => Rotation::Left(count),
                "R" => Rotation::Right(count),
                _ => panic!("Not a valid thing"),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let sequence = parse_input(input);
    let mut accum: u64 = 0;
    let mut current_loc: i32 = 50;

    for single_move in sequence {
        match single_move {
            Rotation::Left(count) => {
                let count: i32 = count as i32 % 100;
                current_loc -= count;
                if current_loc < 0 {
                    current_loc = 100 - (current_loc.abs() % 100);
                }
            }
            Rotation::Right(count) => {
                let count: i32 = count as i32 % 100;
                current_loc += count;
                if current_loc >= 100 {
                    current_loc %= 100;
                }
            }
        }
        if current_loc == 0 {
            accum += 1;
        }
    }

    Some(accum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sequence = parse_input(input);
    let mut accum: u64 = 0;
    let mut current_loc: i32 = 50;

    for single_move in sequence {
        let old_loc = current_loc;
        match single_move {
            Rotation::Left(count) => {
                accum += count as u64 / 100;
                let count: i32 = count as i32 % 100;
                // Force to be between -100 and 99
                current_loc -= count;
                if current_loc < 0 {
                    current_loc += 100;
                    if old_loc != 0 {
                        accum += 1
                    }
                } else if current_loc == 0 && old_loc != 0 {
                    accum += 1
                }
            }
            Rotation::Right(count) => {
                accum += count as u64 / 100;
                let count: i32 = count as i32 % 100;
                current_loc += count;
                if current_loc >= 100 {
                    accum += 1;
                    current_loc -= 100;
                }
            }
        }
    }

    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

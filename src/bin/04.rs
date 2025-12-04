advent_of_code::solution!(4);

#[derive(Debug)]
pub enum Tile {
    Clear,
    Roll,
}

pub fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Clear,
                    '@' => Tile::Roll,
                    _ => panic!("Invalid characted detected"),
                })
                .collect()
        })
        .collect()
}

pub fn is_accessible(x: usize, y: usize, grid: &[Vec<Tile>]) -> u64 {
    let mut number_of_roll_around: usize = 0;
    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;
    // Check the top
    if y > 0 {
        if x > 0 {
            number_of_roll_around += match grid[y - 1][x - 1] {
                Tile::Clear => 0,
                Tile::Roll => 1,
            }
        }
        number_of_roll_around += match grid[y - 1][x] {
            Tile::Clear => 0,
            Tile::Roll => 1,
        };
        if x < max_x {
            number_of_roll_around += match grid[y - 1][x + 1] {
                Tile::Clear => 0,
                Tile::Roll => 1,
            }
        }
    }
    // Check the bottom
    if y < max_y {
        if x > 0 {
            number_of_roll_around += match grid[y + 1][x - 1] {
                Tile::Clear => 0,
                Tile::Roll => 1,
            }
        }
        number_of_roll_around += match grid[y + 1][x] {
            Tile::Clear => 0,
            Tile::Roll => 1,
        };
        if x < max_x {
            number_of_roll_around += match grid[y + 1][x + 1] {
                Tile::Clear => 0,
                Tile::Roll => 1,
            }
        }
    }
    // Check left
    if x > 0 {
        number_of_roll_around += match grid[y][x - 1] {
            Tile::Clear => 0,
            Tile::Roll => 1,
        };
    }
    // Check Right
    if x < max_x {
        number_of_roll_around += match grid[y][x + 1] {
            Tile::Clear => 0,
            Tile::Roll => 1,
        };
    }

    (number_of_roll_around < 4) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut number_of_acc: u64 = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            number_of_acc += match tile {
                Tile::Clear => 0,
                Tile::Roll => is_accessible(x, y, &grid),
            }
        }
    }
    Some(number_of_acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut number_of_removed: u64 = 0;
    loop {
        let mut to_remove: Vec<(usize, usize)> = vec![];
        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let will_be_removed = match tile {
                    Tile::Clear => 0,
                    Tile::Roll => is_accessible(x, y, &grid),
                };
                if will_be_removed == 1 {
                    to_remove.push((x, y));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        number_of_removed += to_remove.len() as u64;
        for (x, y) in to_remove {
            grid[y][x] = Tile::Clear;
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

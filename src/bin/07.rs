use std::collections::HashMap;

advent_of_code::solution!(7);

const CLEAR: u8 = 0;
const SPLITTER: u8 = 1;
const BEAM: u8 = 1;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u8>,
    start_col: usize,
}

pub fn parse_input(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let cols = lines[0].len();
    let mut start_col = 0;

    let real_width = cols + 2;

    let mut data = vec![CLEAR; real_width * height];

    for (y, line) in lines.iter().enumerate() {
        for (x, col) in line.chars().enumerate() {
            if col == 'S' {
                data[y * real_width + (x + 1)] = BEAM;
                start_col = x + 1;
            } else if col == '.' {
                data[y * real_width + (x + 1)] = CLEAR;
            } else if col == '^' {
                data[y * real_width + (x + 1)] = SPLITTER;
            }
        }
    }
    Grid {
        width: real_width,
        height,
        data,
        start_col,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut activated_spliters = 0;
    let mut current_beams: Vec<u8> = vec![CLEAR; grid.width];
    current_beams[grid.start_col] = BEAM;
    // Parse line by line, excluding the first line
    for row in 0..grid.height {
        let mut next_beams: Vec<u8> = vec![CLEAR; grid.width];
        for col in 1..grid.width - 1 {
            let idx = row * grid.width + col;
            if current_beams[col] == BEAM {
                if grid.data[idx] == CLEAR {
                    next_beams[col] = BEAM;
                } else if grid.data[idx] == SPLITTER {
                    next_beams[col - 1] = BEAM;
                    next_beams[col + 1] = BEAM;
                    activated_spliters += 1;
                }
            }
        }
        current_beams = next_beams;
    }
    Some(activated_spliters)
}

pub fn generate_timelines(entry_point: usize, grid: &Grid, cache: &mut HashMap<usize, u64>) -> u64 {
    let mut current_beams: Vec<u8> = vec![CLEAR; grid.width];
    let starting_row = entry_point / grid.width;
    let starting_col = entry_point % grid.width;
    current_beams[entry_point % grid.width] = BEAM;
    if let Some(timelines) = cache.get(&entry_point) {
        return *timelines;
    }
    // We make the beam advance until the next splitter
    for row in starting_row + 1..grid.height {
        let idx = row * grid.width + starting_col;
        if grid.data[idx] == SPLITTER {
            let left_idx = idx + grid.width - 1;
            let right_idx = idx + grid.width + 1;
            let timelines = generate_timelines(left_idx, grid, cache)
                + generate_timelines(right_idx, grid, cache);
            cache.insert(entry_point, timelines);
            return timelines;
        }
    }
    1
}
pub fn generate_timelines_iterative(grid: &Grid) -> u64 {
    // A cache for the number of timelines originating from each grid cell
    // Initialize with 1 for the 'end of the grid' base case, or use Option<u64>
    let mut cache = vec![0; grid.height * grid.width];

    // Iterate backwards through the grid rows
    // Loop from the last row (height - 1) up to row 0
    for row in (0..grid.height).rev() {
        for col in 0..grid.width {
            let idx = row * grid.width + col;

            if grid.data[idx] == SPLITTER {
                // If it's a splitter, the timelines is the sum of timelines
                // from the two cells *below and to the left/right*
                let left_idx = (row + 1) * grid.width + (col as isize - 1) as usize;
                let right_idx = (row + 1) * grid.width + (col as isize + 1) as usize;

                // Safely check bounds and use cached values
                let left_timelines = if col > 0 && row + 1 < grid.height {
                    cache[left_idx]
                } else {
                    1
                };
                let right_timelines = if col < grid.width - 1 && row + 1 < grid.height {
                    cache[right_idx]
                } else {
                    1
                };

                cache[idx] = left_timelines + right_timelines;
            } else {
                // If it's not a splitter, the beam continues straight down
                // The timelines are the timelines from the cell directly *below*
                if row + 1 < grid.height {
                    let next_idx = (row + 1) * grid.width + col;
                    cache[idx] = cache[next_idx];
                } else {
                    // Base case: If it's the bottom row and not a splitter, one timeline
                    cache[idx] = 1;
                }
            }
        }
    }

    // The result is the timeline count starting from the initial entry point
    cache[grid.start_col]
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut cache = HashMap::new();
    Some(generate_timelines(grid.start_col, &grid, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

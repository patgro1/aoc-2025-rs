advent_of_code::solution!(12);

use std::collections::BTreeMap;

fn parse_input(input: &str) -> (Vec<usize>, Vec<((usize, usize), Vec<u64>)>) {
    let mut shape_counts_map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut data_entries: Vec<((usize, usize), Vec<u64>)> = Vec::new();

    // Track which shape we are currently counting hash marks for
    let mut current_shape_id: Option<usize> = None;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Check for a separator (':') which denotes a header
        if let Some(pos) = line.find(':') {
            let header_part = &line[..pos];
            let content_part = &line[pos + 1..];

            if header_part.contains('x') {
                // CASE: Data Line (e.g., "50x39: 42 56...")
                // We are no longer parsing a shape grid
                current_shape_id = None;

                // 1. Parse Dimensions
                let dims: Vec<&str> = header_part.split('x').collect();
                let width = dims[0].parse().unwrap_or(0);
                let height = dims[1].parse().unwrap_or(0);

                // 2. Parse Values
                let values: Vec<u64> = content_part
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                data_entries.push(((width, height), values));
            } else {
                // CASE: Shape Header (e.g., "0:")
                if let Ok(id) = header_part.parse::<usize>() {
                    current_shape_id = Some(id);
                    // Ensure the entry exists (init to 0)
                    shape_counts_map.entry(id).or_insert(0);
                }
            }
        } else {
            // CASE: Shape Grid Body (e.g., ".##", "###")
            // If we are currently inside a shape block, count the '#'
            if let Some(id) = current_shape_id {
                let hashes = line.chars().filter(|&c| c == '#').count();
                *shape_counts_map.entry(id).or_default() += hashes;
            }
        }
    }

    // Convert the BTreeMap to a Vec<usize> where index = shape_id
    // We assume ids are 0-indexed and contiguous based on the prompt
    let max_id = *shape_counts_map.keys().max().unwrap_or(&0);
    let mut shape_counts = vec![0; max_id + 1];
    for (id, count) in shape_counts_map {
        shape_counts[id] = count;
    }

    (shape_counts, data_entries)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shape_counts, trees) = parse_input(input);
    Some(
        trees
            .iter()
            .map(|t| {
                let total_needed_area: u64 =
                    t.1.iter()
                        .zip(shape_counts.iter())
                        .map(|(x, y)| x * *y as u64)
                        .sum();
                let available_area = (t.0.0 * t.0.1).try_into().unwrap();
                // Trivial case where we cannot fit everything due to area
                if total_needed_area >= available_area {
                    return 0;
                }
                let x_grid_3: u64 = (t.0.0) as u64 / 3;
                let y_grid_3: u64 = (t.0.1) as u64 / 3;
                let total_num_of_3by3 = x_grid_3 * y_grid_3;
                if total_num_of_3by3 >= t.1.iter().sum() {
                    return 1;
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

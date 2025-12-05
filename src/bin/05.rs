advent_of_code::solution!(5);
use rayon::prelude::*;

pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut parsing_ranges = true;
    let mut ranges = vec![];
    let mut ingredients = vec![];
    for line in input.lines() {
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let mut range_bounds = line.split('-');
            let beg = range_bounds
                .next()
                .expect("There should be something")
                .parse::<u64>()
                .expect("This should be a number");
            let end = range_bounds
                .next()
                .expect("There should be something")
                .parse::<u64>()
                .expect("This should be a number");
            ranges.push((beg, end));
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    ranges.par_sort_unstable_by(|a, b| a.0.cmp(&b.0));
    (ranges, ingredients)
}

pub fn merge_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut range_iter = ranges.iter().peekable();
    let mut new_ranges = Vec::<(u64, u64)>::with_capacity(ranges.len());
    while let Some(merged_range) = range_iter.next() {
        let mut merged_range = *merged_range;
        while let Some(next_range) = range_iter.peek() {
            if next_range.0 <= merged_range.1 + 1 {
                // consume the next
                if merged_range.1 < next_range.1 {
                    merged_range.1 = next_range.1;
                }
                range_iter.next();
            } else {
                break;
            }
        }
        new_ranges.push(merged_range);
    }
    new_ranges
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input);
    let ranges = merge_ranges(ranges);
    Some(
        ingredients
            .iter()
            .map(|x| {
                if ranges.iter().any(|y| *x >= y.0 && *x <= y.1) {
                    1
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    let ranges = merge_ranges(ranges);
    Some(ranges.iter().map(|x| x.1 - x.0 + 1).sum())
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
        assert_eq!(result, Some(14));
    }
}

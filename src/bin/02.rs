advent_of_code::solution!(2);

use rayon::prelude::*;

pub fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .split(',')
        .map(|x| x.split_once('-').expect("We should split in to string"))
        .collect()
}

pub fn check_invalid_ids(ids_range: (&str, &str)) -> u64 {
    let num_id_range: (u64, u64) = (
        ids_range
            .0
            .parse()
            .expect("We should have a number for start"),
        ids_range
            .1
            .parse()
            .expect("We should have a number for end"),
    );
    (num_id_range.0..=num_id_range.1)
        .into_par_iter()
        .map(|x| {
            let id = x.to_string();
            let id_len = id.len();
            if id_len % 2 != 0 {
                return 0;
            }
            let parts = id.split_at(id_len / 2);
            if parts.0 == parts.1 { x } else { 0 }
        })
        .sum()
}

pub fn generate_ids_parallel(global_max: u64, allow_multiple_repeats: bool) -> Vec<u64> {
    // Step 0, check figure out the min and the max value of the input
    let max_digits = global_max.checked_ilog10().unwrap_or(0) + 1;
    // Since a pattern needs to be present at least twice, the biggest pattern will be number of
    // digits / 2. The +1 is safety for edge cases
    let recipe_max_len = (max_digits / 2) + 1;

    // Step 1: generate all the recipes to generate the invalid ids in the range 0 to global_max
    let mut recipes = Vec::new();
    for len in 1..=recipe_max_len {
        // Valid base for a seed len will always be all the number containing the same amount
        // of numbers as the pattern. i.e. if the pattern is 1010, the base will be 1000 -> 9999
        let rule_min = 10_u64.pow(len - 1);
        let rule_max = 10_u64.pow(len) - 1;

        // Number of repetition of a patern
        for k in 2.. {
            if k > 2 && !allow_multiple_repeats {
                break;
            }
            let mut multiplier = 0_u64;
            for i in 0..k {
                if let Some(shift) = 10_u64.checked_pow(i * len) {
                    multiplier += shift;
                } else {
                    multiplier = 0;
                    break;
                }
            }
            if multiplier == 0 {
                break;
            }
            if rule_min
                .checked_mul(multiplier)
                .is_none_or(|v| v > global_max)
            {
                break;
            }
            recipes.push((multiplier, rule_min, rule_max));
        }
    }
    // Now we can go and generate all invalid numbers
    let mut all_ids: Vec<u64> = recipes
        .par_iter()
        .flat_map_iter(|&(multiplier, min, max)| {
            let effective_max = max.min(global_max / multiplier);
            (min..=effective_max).map(move |seed| seed * multiplier)
        })
        .collect();
    all_ids.sort_unstable();
    all_ids.dedup();

    all_ids
}

pub fn solve_by_cheat(ranges: &Vec<(&str, &str)>, allow_multiple_repeats: bool) -> Option<u64> {
    let global_max = ranges
        .iter()
        .map(|(_, x)| x.parse::<u64>().unwrap())
        .max()
        .unwrap_or(0);
    // Generate all invalid ids that will be within the ranges
    let invalid_ids_list = generate_ids_parallel(global_max, allow_multiple_repeats);
    Some(
        ranges
            .par_iter()
            .map(|(start_str, end_str)| {
                let start: u64 = start_str.parse::<u64>().unwrap();
                let end: u64 = end_str.parse::<u64>().unwrap();

                let start_idx = invalid_ids_list.partition_point(|v| *v < start);
                let end_idx = invalid_ids_list.partition_point(|v| *v <= end);

                if start_idx < end_idx {
                    invalid_ids_list[start_idx..end_idx].iter().sum()
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let ids = parse_input(input);
    solve_by_cheat(&ids, false)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ids = parse_input(input);
    solve_by_cheat(&ids, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

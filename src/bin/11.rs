use std::collections::HashMap;
advent_of_code::solution!(11);

pub fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut puzzle: HashMap<&str, Vec<&str>> = HashMap::new();

    input.trim().lines().for_each(|l| {
        let s = l.trim().split_once(": ").unwrap();
        let targets: Vec<&str> = s.1.split_whitespace().collect();
        puzzle.insert(s.0, targets);
    });

    puzzle
}

pub fn number_of_path_to_out<'a>(
    puzzle: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Recursion exit condition
    if node == "out" {
        return 1;
    }

    if let Some(paths) = cache.get(node) {
        return *paths;
    }

    let nodes = puzzle.get(node).expect("Node should exist in the map");
    let paths = nodes
        .iter()
        .map(|n| number_of_path_to_out(puzzle, n, cache))
        .sum();
    cache.insert(node, paths);
    paths
}

pub fn number_of_path_from_srv_to_out<'a>(
    puzzle: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    seen_dac: bool,
    seen_fft: bool,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    // Recursion exit condition
    if node == "out" {
        if seen_dac && seen_fft {
            return 1;
        } else {
            return 0;
        }
    }

    let mut seen_dac = seen_dac;
    let mut seen_fft = seen_fft;

    if node == "dac" {
        seen_dac = true;
    }
    if node == "fft" {
        seen_fft = true;
    }

    if let Some(paths) = cache.get(&(node, seen_dac, seen_fft)) {
        return *paths;
    }

    let nodes = puzzle.get(node).expect("Node should exist in the map");
    let paths = nodes
        .iter()
        .map(|n| number_of_path_from_srv_to_out(puzzle, n, seen_dac, seen_fft, cache))
        .sum();
    cache.insert((node, seen_dac, seen_fft), paths);
    paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = parse_input(input);
    let mut cache: HashMap<&str, u64> = HashMap::new();
    Some(number_of_path_to_out(&puzzle, "you", &mut cache))
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzle = parse_input(input);
    let mut cache: HashMap<(&str, bool, bool), u64> = HashMap::new();
    Some(number_of_path_from_srv_to_out(
        &puzzle, "svr", false, false, &mut cache,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

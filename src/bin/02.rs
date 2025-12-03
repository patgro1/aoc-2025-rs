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

pub fn check_invalid_ids_multiple_size(ids_range: (&str, &str)) -> u64 {
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
    // TODO: parallalize me
    (num_id_range.0..=num_id_range.1)
        .into_par_iter()
        .map(|x| {
            let id = x.to_string();
            let id_len = id.len();
            for split_at in 2..=id_len {
                let mut parts = id
                    .as_bytes()
                    .chunks(id_len / split_at)
                    .map(|c| str::from_utf8(c).unwrap());
                let are_equal = match parts.next() {
                    Some(first) => parts.all(|c| c == first),
                    None => false,
                };
                if are_equal {
                    return x;
                }
            }
            0
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ids = parse_input(input);
    Some(ids.par_iter().map(|x| check_invalid_ids(*x)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ids = parse_input(input);
    Some(
        ids.par_iter()
            .map(|x| check_invalid_ids_multiple_size(*x))
            .sum(),
    )
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

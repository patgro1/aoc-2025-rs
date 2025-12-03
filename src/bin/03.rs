advent_of_code::solution!(3);
use rayon::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

pub fn find_biggest_combo(bank: &[char], combo_size: usize) -> Vec<char> {
    // Since we need the biggest number, we need to find first the biggest number between
    // the start and the last, excluding the last item
    let mut combo: Vec<char> = vec![];
    let bank = bank.to_vec();
    let bank_len = bank.len();
    if bank_len == combo_size {
        return bank.to_vec();
    } else if combo_size == 0 {
        return vec![];
    }
    let max = bank[..bank_len - (combo_size - 1)]
        .iter()
        .max()
        .expect("We should have a max");
    let max_pos = bank[..bank_len - (combo_size - 1)]
        .iter()
        .position(|x| x == max)
        .expect("We found the number already, it must be there");
    combo.push(*max);
    combo.extend(find_biggest_combo(&bank[max_pos + 1..], combo_size - 1));
    combo
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    Some(
        input
            .par_iter()
            .map(|x| {
                let val: String = find_biggest_combo(x, 2).into_iter().collect();
                let val: u64 = val.parse().expect("We should have a int here");
                val
            })
            .sum(),
    )
    // Some(input.iter().map(|x| find_biggest_combo(x, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    Some(
        input
            .par_iter()
            .map(|x| {
                let val: String = find_biggest_combo(x, 12).into_iter().collect();
                let val: u64 = val.parse().expect("We should have a int here");
                val
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

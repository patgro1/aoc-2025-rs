advent_of_code::solution!(3);
use rayon::prelude::*;

#[allow(clippy::needless_range_loop)]
pub fn find_biggest_combo_sum(bytes: &[u8], combo_size: usize, current_sum: u64) -> u64 {
    // Since we need the biggest number, we need to find first the biggest number between
    // the start and the last, excluding the last item
    let bank_len = bytes.len();
    if combo_size == 0 {
        return current_sum;
    }
    if bytes.len() == combo_size {
        let mut final_acc = current_sum;
        for &b in bytes {
            final_acc = final_acc * 10 + (b - 0x30) as u64;
        }
        return final_acc;
    }
    let mut max: u8 = bytes[0];
    let mut max_pos = 0;
    for x in 1..bank_len - (combo_size - 1) {
        if bytes[x] == b'9' {
            max = b'9';
            max_pos = x;
            break;
        }
        if bytes[x] > max {
            max = bytes[x];
            max_pos = x
        }
    }
    find_biggest_combo_sum(
        &bytes[max_pos + 1..],
        combo_size - 1,
        10 * current_sum + (max as u64 - 0x30),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines()
            .map(|x| find_biggest_combo_sum(x.as_bytes(), 2, 0))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines()
            .map(|x| find_biggest_combo_sum(x.as_bytes(), 12, 0))
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

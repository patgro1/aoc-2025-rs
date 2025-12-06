advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev();
    let op_line = lines.next().expect("There should be an op line");
    let ops = op_line.split_whitespace().collect::<Vec<&str>>();
    let mut accumulators: Vec<_> = ops
        .iter()
        .map(|o| match *o {
            "*" => 1,
            "+" => 0,
            _ => panic!("we got something that aint an operator"),
        })
        .collect();

    for line in lines {
        for (idx, num) in line.split_whitespace().enumerate() {
            match ops[idx] {
                "*" => accumulators[idx] *= num.parse::<u64>().expect("We should have a number"),
                "+" => accumulators[idx] += num.parse::<u64>().expect("We should have a number"),
                _ => panic!("we got something that aint an operator"),
            }
        }
    }
    Some(accumulators.iter().sum())
}

#[derive(Debug)]
pub struct Problem {
    op: u8,
    start_index: usize,
    accumulators: Vec<u64>,
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let op_line = lines.next_back().expect("There must be an opline");
    let mut problems: Vec<Problem> = Vec::new();

    for (i, b) in op_line.bytes().enumerate() {
        if b == b'+' || b == b'*' {
            problems.push(Problem {
                op: b,
                start_index: i,
                accumulators: Vec::new(),
            })
        }
    }

    for line in lines {
        let mut current_prob_idx = 0;
        for (idx, b) in line.bytes().enumerate() {
            // Check if the current column is for the next problem right now
            if current_prob_idx + 1 < problems.len()
                && idx >= problems[current_prob_idx + 1].start_index
            {
                current_prob_idx += 1;
            }
            if b.is_ascii_digit() {
                let prob = &mut problems[current_prob_idx];
                let rel_col = idx - prob.start_index;
                if rel_col >= prob.accumulators.len() {
                    prob.accumulators.resize(rel_col + 1, 0);
                }
                prob.accumulators[rel_col] = prob.accumulators[rel_col] * 10 + b as u64 - 0x30u64;
            }
        }
    }

    let mut accum = 0;
    for problem in problems {
        let mut subtotal = match problem.op {
            b'*' => 1,
            b'+' => 0,
            _ => panic!("Op if invalid"),
        };

        for &val in &problem.accumulators {
            if val == 0 {
                continue;
            }
            match problem.op {
                b'*' => subtotal *= val,
                b'+' => subtotal += val,
                _ => panic!("SHould not go here"),
            };
        }
        accum += subtotal;
    }
    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

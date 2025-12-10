use rayon::prelude::*;
use std::collections::VecDeque;

advent_of_code::solution!(10);

pub fn parse_input(input: &str) -> Vec<(u64, Vec<u64>, Vec<u64>)> {
    let mut puzzle = Vec::new();
    for l in input.lines() {
        let mut buttons = Vec::new();
        let mut end_state = 0;
        let spl_l = l.split_once(" ").unwrap();
        let state_str = spl_l.0;
        for c in state_str
            .trim_matches(|c| c == '[' || c == ']')
            .chars()
            .rev()
        {
            if c == '#' {
                end_state <<= 1;
                end_state |= 1;
            } else if c == '.' {
                end_state <<= 1;
            } else {
                panic!("Not a valid char");
            }
        }
        let spl_l = spl_l.1.split_once('{').unwrap();
        let switches_str = spl_l.0.split_whitespace();
        for switch in switches_str {
            let effects = switch.trim_matches('(').trim_matches(')').split(',');
            let mut final_effect = 0_u64;
            for effect in effects {
                let int_effect: u64 = effect.parse().unwrap();
                final_effect |= 1 << int_effect;
            }
            buttons.push(final_effect);
        }
        let joltage = spl_l
            .1
            .trim_matches('{')
            .trim_matches('}')
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        puzzle.push((end_state, buttons, joltage));
    }
    puzzle
}

const MAX_STATE_SIZE: u32 = 16;
pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = parse_input(input);
    Some(
        puzzles
            .par_iter()
            .map(|(final_state, effects, _)| {
                let mut stack: VecDeque<(u64, u64, u64)> = VecDeque::with_capacity(1000);
                let mut visited: Vec<bool> = vec![false; 2u32.pow(MAX_STATE_SIZE) as usize];

                // Intialize the stack with every effect, states at 0 and press at 1
                for effect in effects {
                    stack.push_back((0, *effect, 1));
                }
                while let Some((state, n_effect, step)) = stack.pop_front() {
                    let new_state = state ^ n_effect;
                    if visited[new_state as usize] {
                        continue;
                    } else {
                        visited[new_state as usize] = true;
                    }

                    // println!(
                    //     "Target: {}, State: {}; Effect: {}, New State: {}; Step: {}",
                    //     *final_state, state, n_effect, new_state, step
                    // );
                    if new_state == *final_state {
                        return step;
                    }
                    // We need to not add current effect since it will only reverse what we just
                    // did
                    for effect in effects {
                        stack.push_back((new_state, *effect, step + 1));
                    }
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

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

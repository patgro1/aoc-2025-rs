use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

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
            .rev()
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

pub fn parse_input_p2(input: &str) -> Vec<(Vec<Vec<u64>>, Vec<u16>)> {
    let mut puzzle = Vec::new();
    for l in input.lines() {
        let mut buttons = Vec::new();
        let spl_l = l.split_once(" ").unwrap();
        let spl_l = spl_l.1.split_once('{').unwrap();
        let switches_str = spl_l.0.split_whitespace();
        for switch in switches_str {
            let effects = switch
                .trim_matches('(')
                .trim_matches(')')
                .split(',')
                .map(|p| p.parse().unwrap())
                .collect();
            buttons.push(effects);
        }
        let joltage = spl_l
            .1
            .trim_matches('{')
            .trim_matches('}')
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        puzzle.push((buttons, joltage));
    }
    puzzle
}

pub fn print_matrix(matrix: &[Vec<f64>]) {
    for row in matrix.iter() {
        // println!("{:?}", row);
        // for col in row.iter() {
        // print!(" {} ", col);
        // }
    }
}

pub fn solve_part_two_linear_algebra(effects: &[Vec<u64>], target: &[u16]) -> Option<u64> {
    let matrix_height = target.len();
    let matrix_width = effects.len();

    // 1. Calculate Upper Bounds for each variable (Button)
    // A button cannot be pressed more times than the smallest target it contributes to.
    let mut variable_bounds = vec![u64::MAX; matrix_width];
    for (col, effect_rows) in effects.iter().enumerate() {
        for &row in effect_rows {
            let t = target[row as usize] as u64;
            if t < variable_bounds[col] {
                variable_bounds[col] = t;
            }
        }
    }

    // 2. Build Matrix (Standard)
    let mut matrix: Vec<Vec<f64>> = vec![vec![0_f64; matrix_width + 1]; matrix_height];
    for i in 0..matrix_height {
        for (j, effect) in effects.iter().enumerate() {
            if effect.contains(&(i as u64)) {
                matrix[i][j] = 1.0;
            }
        }
        matrix[i][matrix_width] = target[i] as f64;
    }

    // 3. Gaussian Elimination (Standard)
    let mut pivot_row = 0;
    for c in 0..matrix_width {
        if pivot_row >= matrix_height {
            break;
        }
        let mut best_row = pivot_row;
        for r in pivot_row + 1..matrix_height {
            if matrix[r][c].abs() > matrix[best_row][c].abs() {
                best_row = r;
            }
        }
        if matrix[best_row][c].abs() < 1e-9 {
            continue;
        }
        matrix.swap(pivot_row, best_row);
        let val = matrix[pivot_row][c];
        for x in c..=matrix_width {
            matrix[pivot_row][x] /= val;
        }
        for r in 0..matrix_height {
            if r != pivot_row {
                let factor = matrix[r][c];
                for j in c..=matrix_width {
                    matrix[r][j] -= factor * matrix[pivot_row][j];
                }
            }
        }
        pivot_row += 1;
    }

    // 4. Identify Columns
    let mut pivot_cols = vec![None; matrix_height];
    let mut is_pivot = vec![false; matrix_width];
    let mut free_cols = Vec::new();

    for r in 0..pivot_row {
        let mut c = 0;
        while c < matrix_width && matrix[r][c].abs() < 1e-9 {
            c += 1;
        }
        if c < matrix_width {
            pivot_cols[r] = Some(c);
            is_pivot[c] = true;
        }
    }
    for c in 0..matrix_width {
        if !is_pivot[c] {
            free_cols.push(c);
        }
    }

    // 5. Recursive Search with Dynamic Bounds
    let mut min_total_presses: Option<u64> = None;

    fn search(
        free_idx: usize,
        current_free_vals: &mut Vec<u64>,
        free_cols: &[usize],
        matrix: &[Vec<f64>],
        pivot_cols: &[Option<usize>],
        variable_bounds: &[u64], // Pass bounds
        matrix_width: usize,
        pivot_row: usize,
        best_so_far: &mut Option<u64>,
    ) {
        if free_idx == free_cols.len() {
            // Check solution logic
            let mut current_solution = vec![0.0; matrix_width];
            for (i, &col_idx) in free_cols.iter().enumerate() {
                current_solution[col_idx] = current_free_vals[i] as f64;
            }

            for r in (0..pivot_row).rev() {
                if let Some(p_col) = pivot_cols[r] {
                    let mut val = matrix[r][matrix_width];
                    for &f_col in free_cols {
                        val -= matrix[r][f_col] * current_solution[f_col];
                    }
                    // Validate Integer and Non-Negative
                    if val < -1e-4 || (val.round() - val).abs() > 1e-4 {
                        return;
                    }

                    let rounded = val.round();
                    // Validate against bound for pivot variable too
                    if rounded < 0.0 || rounded as u64 > variable_bounds[p_col] {
                        return;
                    }

                    current_solution[p_col] = rounded;
                }
            }

            let sum: u64 = current_solution.iter().map(|&x| x as u64).sum();
            *best_so_far = Some(best_so_far.map_or(sum, |m| m.min(sum)));
            return;
        }

        // DYNAMIC LOOP LIMIT: Use the pre-calculated bound for this specific column
        let col_idx = free_cols[free_idx];
        let limit = variable_bounds[col_idx];

        for val in 0..=limit {
            current_free_vals.push(val);
            search(
                free_idx + 1,
                current_free_vals,
                free_cols,
                matrix,
                pivot_cols,
                variable_bounds,
                matrix_width,
                pivot_row,
                best_so_far,
            );
            current_free_vals.pop();
        }
    }

    let mut initial_vals = Vec::new();
    search(
        0,
        &mut initial_vals,
        &free_cols,
        &matrix,
        &pivot_cols,
        &variable_bounds,
        matrix_width,
        pivot_row,
        &mut min_total_presses,
    );

    min_total_presses
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = parse_input_p2(input);
    Some(
        puzzles
            .par_iter()
            .map(|(effects, final_joltage)| {
                // puzzle_idx += 1;
                // println!("Puzzle Idx {puzzle_idx}");
                match solve_part_two_linear_algebra(effects, final_joltage) {
                    Some(val) => val,
                    None => panic!("Puzzle does not have a solution"),
                }
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}

advent_of_code::solution!(8);
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use ::rayon::prelude::*;

pub fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .map(|s| {
            let mut split = s.split(',').map(|x| x.parse::<i64>().unwrap());
            let x = split.next().unwrap();
            let y = split.next().unwrap();
            let z = split.next().unwrap();
            (x, y, z)
        })
        .collect()
}

pub fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] == i {
        return i;
    }
    parent[i] = find(parent[i], parent);
    parent[i]
}

pub fn union(i: usize, j: usize, parent: &mut Vec<usize>, sizes: &mut [usize]) -> bool {
    let root_i = find(i, parent);
    let root_j = find(j, parent);

    if root_i != root_j {
        if sizes[root_i] > sizes[root_j] {
            parent[root_j] = root_i;
            sizes[root_i] += sizes[root_j];
        } else {
            parent[root_i] = root_j;
            sizes[root_j] += sizes[root_i];
        }
        return true;
    }
    false
}

pub fn is_joint(input_len: usize, parents: &mut Vec<usize>) -> bool {
    if input_len == 0 {
        return true;
    };
    let reference_root = find(0, parents);
    for i in 1..input_len {
        if find(i, parents) != reference_root {
            return false;
        }
    }
    return true;
}

#[derive(Eq, PartialEq, Debug)]
pub struct Edge {
    distance: i64,
    i: usize,
    j: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let input_len = input.len();
    const K: usize = 1000;
    let final_heap: BinaryHeap<Edge> = input
        .par_iter()
        .enumerate()
        .flat_map(|(i, ib_ref)| {
            let ib = *ib_ref;
            let input_ref = &input;
            (i + 1..input_len).into_par_iter().map(move |j| {
                let jb = input_ref[j];
                let x_sub = ib.0 - jb.0;
                let y_sub = ib.1 - jb.1;
                let z_sub = ib.2 - jb.2;
                let sq_distance = x_sub * x_sub + y_sub * y_sub + z_sub * z_sub;
                Edge {
                    distance: sq_distance,
                    i,
                    j,
                }
            })
        })
        .fold(
            || BinaryHeap::with_capacity(K + 1),
            |mut heap, edge| {
                if heap.len() < K {
                    heap.push(edge);
                } else if edge.distance < heap.peek().unwrap().distance {
                    heap.pop();
                    heap.push(edge);
                }
                heap
            },
        )
        .reduce(
            || BinaryHeap::new(),
            |mut h1, h2| {
                for edge in h2.into_iter() {
                    if h1.len() < K {
                        h1.push(edge);
                    } else if h1.peek().is_some() && edge.distance < h1.peek().unwrap().distance {
                        h1.pop();
                        h1.push(edge)
                    }
                }
                h1
            },
        );
    let mut parents: Vec<usize> = (0..input_len).collect();
    let mut sizes: Vec<usize> = vec![1; input.len()];
    for edge in final_heap.iter() {
        if find(edge.i, &mut parents) != find(edge.j, &mut parents) {
            union(edge.i, edge.j, &mut parents, &mut sizes);
        }
    }

    let mut circuits_sizes: HashMap<usize, usize> = HashMap::new();
    let n = parents.len();

    for node_index in 0..n {
        let root = find(node_index, &mut parents);
        *circuits_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = circuits_sizes.values().cloned().collect();
    sizes.sort_by(|a, b| b.cmp(a));
    let mut res: u64 = 1;
    sizes.iter().take(3).for_each(|x| res *= (*x) as u64);

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);
    let input_len = input.len();
    let mut distances: Vec<Edge> = input
        .par_iter()
        .enumerate()
        .flat_map(|(i, ib_ref)| {
            let ib = *ib_ref;
            let input_ref = &input;
            (i + 1..input_len).into_par_iter().map(move |j| {
                let jb = input_ref[j];
                let x_sub = ib.0 - jb.0;
                let y_sub = ib.1 - jb.1;
                let z_sub = ib.2 - jb.2;
                let sq_distance = x_sub * x_sub + y_sub * y_sub + z_sub * z_sub;
                Edge {
                    distance: sq_distance,
                    i,
                    j,
                }
            })
        })
        .collect();
    distances.sort_by(|a, b| a.distance.cmp(&b.distance));

    let mut parents: Vec<usize> = (0..input_len).collect();
    let mut sizes: Vec<usize> = vec![1; input.len()];
    let mut edge_iter = distances.iter();
    let mut mult: u64 = 0;
    let mut components = input.len();
    while components > 1 {
        let edge = edge_iter.next().expect("We must have an edge");
        if find(edge.i, &mut parents) != find(edge.j, &mut parents) {
            union(edge.i, edge.j, &mut parents, &mut sizes);
            components -= 1;
        }
        if components == 1 {
            mult = (input[edge.i].0 * input[edge.j].0).try_into().unwrap();
            break;
        }
    }
    Some(mult)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

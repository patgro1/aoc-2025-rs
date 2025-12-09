use rayon::prelude::*;

advent_of_code::solution!(9);
type Point = (i64, i64);

pub struct Boundary {
    pub v_segments: Vec<(i64, i64, i64)>,
    pub h_segments: Vec<(i64, i64, i64)>,
    pub all_points: Vec<Point>,
}

pub fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let split = l.split_once(',').unwrap();
            (split.0.parse().unwrap(), split.1.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let num_points = points.len();
    Some(
        points
            .par_iter()
            .enumerate()
            .flat_map(|(i, p_ref)| {
                let p_a = p_ref;
                let points_ref = &points;
                (i + 1..num_points).into_par_iter().map(move |j| {
                    let p_b = &points_ref[j];
                    let x_sub = (p_a.0 - p_b.0).abs() + 1;
                    let y_sub = (p_a.1 - p_b.1).abs() + 1;
                    x_sub * y_sub
                })
            })
            .max()
            .unwrap()
            .try_into()
            .expect("The number should be positive"),
    )
}

pub fn parse_boundary(points: &[Point]) -> Boundary {
    let mut v_segments = Vec::new();
    let mut h_segments = Vec::new();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        if p1.0 == p2.0 {
            v_segments.push((p1.0, p1.1.min(p2.1), p1.1.max(p2.1)));
        } else {
            h_segments.push((p1.1, p1.0.min(p2.0), p1.0.max(p2.0)));
        }
    }

    Boundary {
        v_segments,
        h_segments,
        all_points: points.to_vec(),
    }
}

pub fn edges_intersect(x1: i64, y1: i64, x2: i64, y2: i64, boundary: &Boundary) -> bool {
    for &ry in &[y1, y2] {
        for &(bx, by_min, by_max) in &boundary.v_segments {
            if bx > x1 && bx < x2 && ry > by_min && ry < by_max {
                return true;
            }
        }
    }
    for &rx in &[x1, x2] {
        for &(by, bx_min, bx_max) in &boundary.h_segments {
            if by > y1 && by < y2 && rx > bx_min && rx < bx_max {
                return true;
            }
        }
    }
    false
}

// Ray Casting: Odd = Inside
fn ray_cast(x: f64, y: f64, boundary: &Boundary) -> bool {
    let mut intersections = 0;
    for &(vx, vy_min, vy_max) in &boundary.v_segments {
        let vx_f = vx as f64;
        let vy_min_f = vy_min as f64;
        let vy_max_f = vy_max as f64;

        // Check if Y is within range
        if y > vy_min_f && y < vy_max_f {
            // Check if wall is to the right
            if vx_f > x {
                intersections += 1;
            }
        }
    }
    intersections % 2 != 0
}

fn is_center_inside(x1: i64, y1: i64, x2: i64, y2: i64, boundary: &Boundary) -> bool {
    let cx = (x1 as f64 + x2 as f64) / 2.0;
    let cy = (y1 as f64 + y2 as f64) / 2.0;
    ray_cast(cx, cy, boundary)
}

fn vertex_inside(x1: i64, y1: i64, x2: i64, y2: i64, boundary: &Boundary) -> bool {
    for p in &boundary.all_points {
        if p.0 > x1 && p.0 < x2 && p.1 > y1 && p.1 < y2 {
            return true;
        }
    }
    false
}

// [NEW] Check if a point is literally on top of a boundary wall
fn is_on_boundary(x: f64, y: f64, boundary: &Boundary) -> bool {
    // Check vertical walls
    for &(vx, vy_min, vy_max) in &boundary.v_segments {
        if (x - vx as f64).abs() < 1e-9 && y >= vy_min as f64 && y <= vy_max as f64 {
            return true;
        }
    }
    // Check horizontal walls
    for &(hy, hx_min, hx_max) in &boundary.h_segments {
        if (y - hy as f64).abs() < 1e-9 && x >= hx_min as f64 && x <= hx_max as f64 {
            return true;
        }
    }
    false
}

// [NEW] Validates that an edge doesn't bridge a gap
fn validate_rect_edge(p1: Point, p2: Point, boundary: &Boundary) -> bool {
    let (x_start, x_end) = (p1.0.min(p2.0), p1.0.max(p2.0));
    let (y_start, y_end) = (p1.1.min(p2.1), p1.1.max(p2.1));

    // 1. Find all polygon vertices that lie on this segment
    let mut stops = vec![0.0];
    let len = ((x_end - x_start) + (y_end - y_start)) as f64;
    stops.push(len);

    for p in &boundary.all_points {
        // If point is on the segment
        if p.0 >= x_start && p.0 <= x_end && p.1 >= y_start && p.1 <= y_end {
            // Calculate distance from start to sort properly
            let dist = ((p.0 - p1.0).abs() + (p.1 - p1.1).abs()) as f64;
            if dist > 0.0 && dist < len {
                stops.push(dist);
            }
        }
    }
    stops.sort_by(|a, b| a.partial_cmp(b).unwrap());
    stops.dedup();

    // 2. Check the midpoint of every sub-segment defined by these vertices
    for i in 0..stops.len() - 1 {
        let mid_dist = (stops[i] + stops[i + 1]) / 2.0;

        // Determine coordinate of midpoint
        let mx: f64;
        let my: f64;

        if x_start == x_end {
            // Vertical Edge
            // p1.1 is start Y if p1 is the "bottom" relative to sort?
            // Actually simpler: interpolate
            let ratio = mid_dist / len;
            mx = p1.0 as f64;
            my = p1.1 as f64 + (p2.1 - p1.1) as f64 * ratio;
        } else {
            // Horizontal Edge
            let ratio = mid_dist / len;
            mx = p1.0 as f64 + (p2.0 - p1.0) as f64 * ratio;
            my = p1.1 as f64;
        }

        // 3. The Test: Is the midpoint on a wall OR inside the lava?
        if !is_on_boundary(mx, my, boundary) && !ray_cast(mx, my, boundary) {
            return false; // Found a gap!
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_input(input);
    let boundary = parse_boundary(&points);
    let num_points = points.len();
    let mut rectangles: Vec<((i64, i64, i64, i64), i64)> = points
        .par_iter()
        .enumerate()
        .flat_map(|(i, p_ref)| {
            let p_a = p_ref;
            let points_ref = &points;
            (i + 1..num_points).into_par_iter().map(move |j| {
                let p_b = &points_ref[j];
                let x1 = p_a.0.min(p_b.0);
                let x2 = p_a.0.max(p_b.0);
                let y1 = p_a.1.min(p_b.1);
                let y2 = p_a.1.max(p_b.1);

                // If collinear (line not rect), area is 0 (or width 1 * length)
                // Problem says "rectangle", implies area > 0 usually?
                // Assuming standard area calculation:
                let x_sub = x2 - x1 + 1;
                let y_sub = y2 - y1 + 1;
                ((x1, y1, x2, y2), x_sub * y_sub)
            })
        })
        .collect();

    rectangles.par_sort_by(|a, b| b.1.cmp(&a.1));

    for ((x1, y1, x2, y2), area) in rectangles {
        // Skip lines if needed, or process them
        if x1 == x2 || y1 == y2 {
            continue;
        }

        if edges_intersect(x1, y1, x2, y2, &boundary) {
            continue;
        }
        if vertex_inside(x1, y1, x2, y2, &boundary) {
            continue;
        }

        // [NEW] Check all 4 edges for "Bridging Gaps"
        if !validate_rect_edge((x1, y1), (x2, y1), &boundary) {
            continue;
        } // Top
        if !validate_rect_edge((x1, y2), (x2, y2), &boundary) {
            continue;
        } // Bottom
        if !validate_rect_edge((x1, y1), (x1, y2), &boundary) {
            continue;
        } // Left
        if !validate_rect_edge((x2, y1), (x2, y2), &boundary) {
            continue;
        } // Right

        if is_center_inside(x1, y1, x2, y2, &boundary) {
            return Some(area.try_into().unwrap());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn print_rope(positions: &mut Vec<Position>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for position in positions.iter() {
        if position.x < min_x {
            min_x = position.x;
        }
        if position.x > max_x {
            max_x = position.x;
        }
        if position.y < min_y {
            min_y = position.y;
        }
        if position.y > max_y {
            max_y = position.y;
        }
    }
    for y in min_y - 2..=max_y + 2 {
        for x in min_x - 2..=max_x + 2 {
            let position = Position { x, y };
            match &positions.iter().position(|p| *p == position) {
                Some(idx) => print!("{}", idx),
                None => print!("."),
            }
        }
        println!();
    }
    &positions.iter().enumerate().for_each(|(idx, position)| {
        print!("{} - {:?}, ", idx, position);
    });
    println!("\n");
}

/*
 * Moves a knot, returns where next knot should move
 */
fn move_knot(positions: &mut Vec<Position>, knot_idx: usize, dx: i32, dy: i32) -> (i32, i32) {
    // tricky way to get two mut refs from vector
    let (heads, tails) = positions.split_at_mut(knot_idx + 1);
    let head_position = &mut heads[knot_idx];
    let old_tail_position = &tails[0];
    let mut new_tail_position = old_tail_position.clone();

    head_position.x += dx;
    head_position.y += dy;

    // next knot is still adjacent
    if (head_position.x - old_tail_position.x).abs() <= 1
        && (head_position.y - old_tail_position.y).abs() <= 1
    {
        return (0, 0);
    }

    // diagonal move
    if dx != 0 && dy != 0 {
        if head_position.x != old_tail_position.x {
            new_tail_position.x += dx;
        }
        if head_position.y != old_tail_position.y {
            new_tail_position.y += dy;
        }
    } else {
        // standard move (up, down, left, right)
        if (head_position.x - old_tail_position.x).abs() > 1 {
            new_tail_position.x += dx;
            new_tail_position.y = head_position.y
        } else if (head_position.y - old_tail_position.y).abs() > 1 {
            new_tail_position.x = head_position.x;
            new_tail_position.y += dy
        }
    }

    (
        new_tail_position.x - old_tail_position.x,
        new_tail_position.y - old_tail_position.y,
    )
}

fn solve<T: std::io::Read>(reader: BufReader<T>, knots_amount: usize) -> HashSet<Position> {
    let mut positions = vec![Position { x: 0, y: 0 }; knots_amount];
    let mut positions_visited = HashSet::new(); // by tail == positions.last()

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, count) = line.split_once(" ").unwrap();

        let count = count.parse::<u32>().unwrap();
        let (dx_head, dy_head) = match direction {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Unknown direction {}", direction),
        };

        // repeat move required no of times
        for _ in 0..count {
            // move head and first knot, then first know and second knot, etc
            let (mut dx, mut dy) = (dx_head, dy_head);
            for knot_to_move_as_head in 0..positions.len() - 1 {
                let (dx_new, dy_new) = move_knot(&mut positions, knot_to_move_as_head, dx, dy);
                dx = dx_new;
                dy = dy_new;
            }
            let tail = positions.last_mut().unwrap();
            tail.x += dx;
            tail.y += dy;
            positions_visited.insert(tail.clone());
        }
        // print_rope(&mut positions);
    }
    positions_visited
}

fn main() {
    {
        let file = File::open("./src/day9/input.txt").unwrap();
        let reader = BufReader::new(file);
        println!("Puzzle1: {}", solve(reader, 2).len());
    }
    {
        let file = File::open("./src/day9/input.txt").unwrap();
        let reader = BufReader::new(file);
        println!("Puzzle2: {}", solve(reader, 10).len());
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_simple() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(
            solve(std::io::BufReader::new(input.as_bytes()), 2).len(),
            13
        );
        assert_eq!(solve(std::io::BufReader::new(input.as_bytes()), 9).len(), 1);
    }

    #[test]
    fn test_big() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(
            solve(std::io::BufReader::new(input.as_bytes()), 10).len(),
            36
        );
    }
}

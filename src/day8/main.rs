use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

// compact set for puzzle1
struct BitArray {
    data: Vec<u8>,
    size_x: usize,
    size_y: usize,
}

impl BitArray {
    fn new(bit_size_x: usize, bit_size_y: usize) -> Self {
        let data = vec![0; (bit_size_x * bit_size_y + 7) / 8];
        Self {
            data,
            size_x: bit_size_x,
            size_y: bit_size_y,
        }
    }

    fn set_bit(&mut self, x: usize, y: usize) {
        let index = x + y * self.size_x;
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.data[byte_index] |= 1 << bit_index;
    }

    fn check_bit(&self, x: usize, y: usize) -> bool {
        let index = x + y * self.size_x;
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.data[byte_index] & (1 << bit_index) != 0
    }

    fn sum_bits(&self) -> usize {
        self.data
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }
}

impl Display for BitArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.check_bit(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// easy reading of input
fn input_to_forest<T: std::io::Read>(reader: BufReader<T>) -> Vec<Vec<i8>> {
    let mut forest = Vec::new();

    // assumes all lines are the same length
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::with_capacity(line.len());
        for tree in line.chars() {
            row.push(tree.to_digit(10).unwrap() as i8);
        }
        forest.push(row);
    }
    forest
}

/*
 * Puzzle 1: set a bit corresponding to a tree if the tree is visible
 */
fn puzzle1_set_visible_trees(bit_array: &mut BitArray, forest: &Vec<Vec<i8>>) {
    let mut highest_from_top = vec![-1; forest[0].len()];
    let mut highest_from_left = vec![-1; forest.len()];
    for y in 0..forest.len() {
        for x in 0..forest[y].len() {
            let current_tree = forest[y][x];
            if current_tree > highest_from_top[x] {
                highest_from_top[x] = current_tree;
                bit_array.set_bit(x, y);
            }
            if current_tree > highest_from_left[y] {
                highest_from_left[y] = current_tree;
                bit_array.set_bit(x, y);
            }
        }
    }

    // now from bottom-right
    let mut highest_from_bottom = vec![-1; forest[0].len()];
    let mut highest_from_right = vec![-1; forest.len()];
    for (y, row) in forest.iter().enumerate().rev() {
        for (x, tree) in row.iter().enumerate().rev() {
            if *tree > highest_from_bottom[x] {
                highest_from_bottom[x] = *tree;
                bit_array.set_bit(x, y);
            }
            if *tree > highest_from_right[y] {
                highest_from_right[y] = *tree;
                bit_array.set_bit(x, y);
            }
        }
    }
}

/*
 * Puzzle 2: given single line of trees, compute score of every tree
 * Takes closure as first as, bcause we want to iterate forest in various directions
 */
fn score_of_trees_one_line<F>(get_tree_at_index: F, line_size: usize) -> Vec<usize>
where
    F: Fn(usize) -> i8,
{
    let mut scores: Vec<usize> = Vec::with_capacity(line_size);
    scores.push(0);

    for current_tree_idx in 1..line_size {
        let current_tree = get_tree_at_index(current_tree_idx);

        // compare current tree to all previous trees
        let mut cmp_tree_idx = current_tree_idx - 1;
        while cmp_tree_idx > 0 {
            // we found a tree that is higher or equal to the current tree
            if get_tree_at_index(cmp_tree_idx) >= current_tree {
                break;
            }
            // we found a tree that is lower than the current tree
            // we can jump over to the tree that is higher or equal to the tree we found
            cmp_tree_idx -= scores[cmp_tree_idx];
        }

        scores.push(current_tree_idx - cmp_tree_idx);
    }
    scores
}

/*
 * Puzzle 2: given a forest, compute score of every tree
 * Simply call score_of_trees_one_line for every line in all 4 directions
 */
fn score_of_forest(forest: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
    let mut scores_puzzle2 = vec![vec![1 as usize; forest[0].len()]; forest.len()];

    forest
        .iter()
        .zip(&mut scores_puzzle2)
        .for_each(|(forest_line, scores_line)| {
            // left-right score
            score_of_trees_one_line(|idx| forest_line[idx], forest_line.len())
                .iter()
                .zip(scores_line.iter_mut())
                .for_each(|(score_new, score_old)| {
                    *score_old *= score_new;
                });

            // right-left score
            score_of_trees_one_line(
                |idx| forest_line[forest_line.len() - idx - 1],
                forest_line.len(),
            )
            .iter()
            .rev()
            .zip(scores_line)
            .for_each(|(score_new, score_old)| {
                *score_old *= score_new;
            });
        });

    for line_idx in 0..forest[0].len() {
        // top-bottom score
        score_of_trees_one_line(|idx| forest[idx][line_idx], forest.len())
            .iter()
            .enumerate()
            .for_each(|(idx, score_new)| {
                scores_puzzle2[idx][line_idx] *= score_new;
            });

        // bottom-top score
        score_of_trees_one_line(|idx| forest[forest.len() - idx - 1][line_idx], forest.len())
            .iter()
            .rev()
            .enumerate()
            .for_each(|(idx, score_new)| {
                scores_puzzle2[idx][line_idx] *= score_new;
            });
    }
    scores_puzzle2
}

fn main() {
    let file = File::open("./src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);
    let forest = input_to_forest(reader);

    let mut visible_trees_map = BitArray::new(forest[0].len(), forest.len());
    puzzle1_set_visible_trees(&mut visible_trees_map, &forest);
    println!("Part 1: {}", visible_trees_map.sum_bits());

    let scores_puzzle2 = score_of_forest(&forest);
    println!(
        "Part 2: {:?}",
        scores_puzzle2.iter().flatten().max().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        input_to_forest, puzzle1_set_visible_trees, score_of_forest, score_of_trees_one_line,
        BitArray,
    };
    use std::io::BufReader;

    #[test]
    fn test_score_of_trees_one_line() {
        let forest_line = vec![2, 6, 1, 2, 2, 3, 5, 8, 6, 11];
        let scores = score_of_trees_one_line(|idx| forest_line[idx], forest_line.len());
        assert_eq!(scores, vec![0, 1, 1, 2, 1, 4, 5, 7, 1, 9]);

        let forest_line2 = vec![9, 5, 6, 2, 4, 1, 1, 8, 11];
        let scores2 = score_of_trees_one_line(|idx| forest_line2[idx], forest_line2.len());
        assert_eq!(scores2, vec![0, 1, 2, 1, 2, 1, 1, 7, 8]);
    }

    #[test]
    fn test_sample() {
        let input = "30373
25512
65332
33549
35390";
        let mut forest = input_to_forest(BufReader::new(input.as_bytes()));
        let mut visible_trees_map = BitArray::new(forest[0].len(), forest.len());
        puzzle1_set_visible_trees(&mut visible_trees_map, &forest);
        println!("{}", visible_trees_map);
        println!("{:?}", forest);
        assert_eq!(visible_trees_map.sum_bits(), 21);

        let score2 = score_of_forest(&forest);
        assert_eq!(
            score2,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 4, 1, 0],
                vec![0, 6, 1, 2, 0],
                vec![0, 1, 8, 3, 0],
                vec![0, 0, 0, 0, 0]
            ]
        );
    }
}

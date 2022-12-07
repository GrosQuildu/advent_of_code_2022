use std::collections::{HashMap, HashSet, VecDeque};

fn insert_or_increment(map: &mut HashMap<u8, usize>, key: u8) {
    let counter = map.entry(key).or_insert(0);
    *counter += 1;
}

fn find_marker(input: Vec<u8>, marker_size: usize) -> Option<usize> {
    if input.len() < marker_size + 1 {
        return None;
    }
    let mut marker_map: HashMap<u8, usize> = HashMap::new();
    for index in 0..marker_size {
        insert_or_increment(&mut marker_map, input[index]);
    }

    let mut last_char_iter = input.iter();
    for (index, a_char) in input.iter().skip(marker_size).enumerate() {
        if marker_map
            .iter()
            .all(|(_, &count)| count == 1 || count == 0)
        {
            return Some(index + marker_size);
        }

        // add newly checked char
        let counter = marker_map.entry(*a_char).or_insert(0);
        *counter += 1;

        // remove old char
        marker_map
            .entry(*last_char_iter.next().unwrap())
            .and_modify(|e| *e -= 1);
    }
    None
}

fn main() {
    let input = std::fs::read("./src/day6/input.txt").unwrap();
    match find_marker(input.clone(), 4) {
        Some(index) => println!("Found marker of size 4 at index {}", index),
        None => println!("No marker of size 4 found"),
    }
    match find_marker(input, 14) {
        Some(index) => println!("Found marker of size 14 at index {}", index),
        None => println!("No marker of size 14 found"),
    }
}

#[test]
fn test_puzzle1() {
    let input = b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let marker = find_marker(input.to_vec(), 4);
    assert_eq!(marker, Some(10));
}

#[test]
fn test_puzzle1_2() {
    let input = b"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let marker = find_marker(input.to_vec(), 14);
    assert_eq!(marker, Some(19));
}

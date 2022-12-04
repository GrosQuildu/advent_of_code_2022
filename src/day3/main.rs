use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rucksack {
    compartment_a: Vec<char>,
    compartment_b: Vec<char>,
}

fn score(x: char) -> Result<u8, &'static str> {
    let xx = x as u8;
    if xx >= 97 && xx <= 122 {
        return Ok(xx - 96);
    } else if xx >= 65 && xx <= 90 {
        return Ok(xx - 38);
    }
    Err("Invalid character")
}

#[test]
fn test_score() {
    assert_eq!(score('a'), Ok(1));
    assert_eq!(score('A'), Ok(27));
    assert_eq!(score('z'), Ok(26));
    assert_eq!(score('Z'), Ok(52));
    assert_eq!(score('0'), Err("Invalid character"));
}

fn main() {
    let file = File::open("./src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rucksacks = Vec::new();

    for linex in reader.lines() {
        let line = linex.unwrap();
        let compartment_separator_index = line.len() / 2;
        let rucksack = Rucksack {
            compartment_a: line[0..compartment_separator_index].chars().collect(),
            compartment_b: line[compartment_separator_index..].chars().collect(),
        };
        rucksacks.push(rucksack);
    }

    let result: u32 = rucksacks
        .iter()
        .map(|r| {
            let a: HashSet<char> = HashSet::from_iter(r.compartment_a.clone());
            let b: HashSet<char> = HashSet::from_iter(r.compartment_b.clone());
            a.intersection(&b).map(|x| score(*x).unwrap()).sum::<u8>() as u32
        })
        .sum();

    println!("Result puzzle 1: {}", result);

    let result2 = rucksacks
        .chunks(3)
        .map(|group| {
            let badges = group
                .iter()
                .map(|r| {
                    let mut a: HashSet<char> = HashSet::from_iter(r.compartment_a.clone());
                    let b: HashSet<char> = HashSet::from_iter(r.compartment_b.clone());
                    a.extend(&b);
                    return a;
                })
                .reduce(|acc, x| acc.intersection(&x).copied().collect())
                .unwrap();
            if badges.len() != 1 {
                panic!("Badges are not unique {}", badges.len());
            }
            let badge = badges.iter().next().unwrap();
            score(*badge).unwrap() as u32
        })
        .sum::<u32>();

    println!("Result puzzle 2: {}", result2);
}

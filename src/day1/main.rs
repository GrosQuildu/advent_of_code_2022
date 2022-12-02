use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fmt::{self, Display};
use std::cmp::Ordering;

struct Ant {
    number: i16,
    calories: Vec<u32>
}

impl Display for Ant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{} - {:?}", self.number, self.calories);
    }
}

impl Ord for Ant {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.sum_calories()).cmp(&(other.sum_calories()))
    }
}

impl PartialOrd for Ant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Ant {
    fn eq(&self, other: &Self) -> bool {
        (&self.number, &self.calories) == (&other.number, &other.calories)
    }
}

impl Eq for Ant { }

impl Ant {
    fn sum_calories(&self) -> u32 {
        return self.calories.iter().sum();
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./src/day1/input1.txt")?;
    let reader = BufReader::new(file);

    let mut idx = 0;
    let mut calories_tmp = Vec::new();
    let mut ants: Vec<Ant> = Vec::new();

    for linex in reader.lines() {
        let line = linex.unwrap();
        if line.is_empty() {
            let ant = Ant{number:idx, calories: calories_tmp.clone()};
            ants.push(ant);
            idx += 1;
            calories_tmp.clear();
            continue;
        }
        calories_tmp.push(line.parse::<u32>().unwrap());
    }

    if !calories_tmp.is_empty() {
        let ant = Ant{number:idx, calories: calories_tmp.clone()};
        ants.push(ant);
    }

    for ant in &ants {
        println!("Ant {}", ant);
    }

    // puzzle 1
    let max_ant = ants.iter().max().unwrap();
    println!("Max ant: {} - {}", max_ant, max_ant.sum_calories());

    // puzzle 2
    ants.sort();
    ants.reverse();
    let sum_three: u32 = ants.iter().take(3).map(|a| a.sum_calories()).sum();
    println!("Max 3 ants sum: {}", sum_three);

    Ok(())
}

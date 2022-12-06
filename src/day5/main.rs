use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline, space1};
use nom::combinator::{eof, opt, recognize};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, terminated};
use nom::{AsChar, IResult};
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::str::from_utf8;

type Crate = char;

#[derive(Debug, PartialEq)]
struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

fn parse_crate(input: &[u8]) -> IResult<&[u8], Option<Crate>> {
    let (input, maybe_crate) = alt((tag("   "), delimited(tag("["), alpha1, tag("]"))))(input)?;
    match maybe_crate {
        b"   " => Ok((input, None)),
        a_crate => Ok((input, Some(a_crate.first().unwrap().as_char()))),
    }
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<Option<Crate>>> {
    let (input, crates_line) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, crates_line))
}

fn parse_crates_lines(input: &[u8]) -> IResult<&[u8], Vec<VecDeque<Crate>>> {
    let (input, crates_lines) = many1(terminated(parse_line, newline))(input)?;

    let (input, crates_numbers) = terminated(
        many1(preceded(space1, recognize(nom::character::complete::u8))),
        newline,
    )(input)?;

    let mut stacks = vec![VecDeque::new(); crates_numbers.len()];
    crates_lines.iter().rev().for_each(|line| {
        line.iter().enumerate().for_each(|(i, maybe_crate)| {
            if let Some(a_crate) = maybe_crate {
                stacks[i].push_front(*a_crate);
            }
        })
    });
    Ok((input, stacks))
}

fn parse_move(i: &[u8]) -> IResult<&[u8], Move> {
    let (input, _) = tag("move ")(i)?;
    let (input, amount) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = digit1(input)?;
    Ok((
        input,
        Move {
            amount: from_utf8(amount).unwrap().parse().unwrap(),
            from: from_utf8(from).unwrap().parse().unwrap(),
            to: from_utf8(to).unwrap().parse().unwrap(),
        },
    ))
}

fn parse_input(input: &[u8]) -> IResult<&[u8], (Vec<VecDeque<Crate>>, Vec<Move>)> {
    let (input, stacks) = parse_crates_lines(input)?;
    let (input, _) = newline(input)?;
    let (input, moves) = many1(terminated(parse_move, opt(newline)))(input)?;
    eof(input)?;
    Ok((input, (stacks, moves)))
}

#[test]
fn test_parse_line() {
    let input = b"    [D]    [X]";
    let expected = vec![None, Some('D'), None, Some('X')];
    assert_eq!(parse_line(input), Ok((&[][..], expected)));
}

#[test]
fn test_parse_crates_lines() {
    let input = b"
    [D]
[N] [C]
[Z] [M] [P]
 1  2  3
";
    let expected: Vec<VecDeque<Crate>> = vec![
        VecDeque::from(vec!['N', 'Z']),
        VecDeque::from(vec!['D', 'C', 'M']),
        VecDeque::from(vec!['P']),
    ];
    assert_eq!(parse_crates_lines(&input[1..]), Ok((&b""[..], expected)));
}

#[test]
fn test_parse_move() {
    let input = b"move 22 from 11 to 3";
    let expected = Move {
        amount: 22,
        from: 11,
        to: 3,
    };
    assert_eq!(parse_move(input), Ok((&b""[..], expected)));
}

fn print_stacks_tops(stacks: Vec<VecDeque<Crate>>) {
    stacks
        .iter()
        .map(|s| *s.front().unwrap())
        .for_each(|c| print!("{}", c));
}

fn main() {
    let input = std::fs::read("./src/day5/input.txt").unwrap();
    let (_, (mut stacks, moves)) = parse_input(&input).unwrap();

    let mut stacks_copy = stacks.clone();
    moves.iter().for_each(|m| {
        for _ in 0..m.amount {
            let c = stacks_copy[(m.from - 1) as usize].pop_front().unwrap();
            stacks_copy[(m.to - 1) as usize].push_front(c);
        }
    });

    print!("Puzzle 1: ");
    print_stacks_tops(stacks_copy);
    println!();

    moves.iter().for_each(|m| {
        let mut c = stacks[(m.from - 1) as usize]
            .drain(..(m.amount as usize))
            .collect::<VecDeque<_>>();
        c.append(&mut stacks[(m.to - 1) as usize]);
        stacks[(m.to - 1) as usize] = c;
    });

    print!("Puzzle 2: ");
    print_stacks_tops(stacks);
    println!();
}

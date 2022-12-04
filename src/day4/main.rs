extern crate gcollections;
extern crate interval;

use crate::interval::ops::*;
use gcollections::ops::set::Overlap;
use interval::Interval;
use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, one_of};
use nom::combinator::{eof, map_res, opt, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

type IntervalPair = (Interval<u16>, Interval<u16>);

fn parse_sections(i: &[u8]) -> IResult<&[u8], Interval<u16>> {
    let (input, (start, _, end)) = tuple((
        nom::character::complete::u16,
        tag("-"),
        nom::character::complete::u16,
    ))(i)?;
    Ok((input, Interval::new(start, end)))
}

fn parse_line(i: &[u8]) -> IResult<&[u8], IntervalPair> {
    let (input, (elf_first, _, elf_second)) = tuple((parse_sections, tag(","), parse_sections))(i)?;
    Ok((input, (elf_first, elf_second)))
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Vec<IntervalPair>> {
    let (input, input_data) = many0(terminated(parse_line, opt(line_ending)))(input)?;
    eof(input)?;
    return Ok((input, input_data));
}

fn main() {
    let input = std::fs::read("./src/day4/input.txt").unwrap();
    let (_, input_data) = parse_input(&input).unwrap();

    let result1 = input_data
        .iter()
        .filter(|(elf_first, elf_second)| {
            let h = Hull::hull(elf_first, elf_second);
            h == *elf_first || h == *elf_second
        })
        .count();

    println!("Result puzzle 1: {}", result1);

    let result2 = input_data
        .iter()
        .filter(|(elf_first, elf_second)| elf_first.overlap(elf_second))
        .count();

    println!("Result puzzle 2: {}", result2);
}

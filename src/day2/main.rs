use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::{eof, map, opt};
use nom::multi::many0;
use nom::sequence::{terminated, tuple};
use nom::IResult;

// game types
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PRSMove {
    Rock,
    Paper,
    Scissors,
}

impl PRSMove {
    fn beats(&self, other: &PRSMove) -> bool {
        match (self, other) {
            (PRSMove::Rock, PRSMove::Scissors) => true,
            (PRSMove::Paper, PRSMove::Rock) => true,
            (PRSMove::Scissors, PRSMove::Paper) => true,
            _ => false,
        }
    }

    // lame, better would be matrix or prolog-like auto matching
    fn get_winning(&self) -> PRSMove {
        match self {
            PRSMove::Rock => PRSMove::Paper,
            PRSMove::Paper => PRSMove::Scissors,
            PRSMove::Scissors => PRSMove::Rock,
        }
    }

    fn get_loosing(&self) -> PRSMove {
        match self {
            PRSMove::Rock => PRSMove::Scissors,
            PRSMove::Paper => PRSMove::Rock,
            PRSMove::Scissors => PRSMove::Paper,
        }
    }

    fn score(&self) -> u8 {
        match self {
            PRSMove::Rock => 1,
            PRSMove::Paper => 2,
            PRSMove::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PRSRound {
    opponent: PRSMove,
    player: PRSMove,
}

impl PRSRound {
    fn score(&self) -> u8 {
        let result = if self.opponent.beats(&self.player) {
            0
        } else if self.player.beats(&self.opponent) {
            6
        } else {
            3
        };
        result + self.player.score()
    }

    fn score_fixed(&self) -> u8 {
        let new_player = match self.player {
            // we need to loose
            PRSMove::Rock => self.opponent.get_loosing(),
            // draw
            PRSMove::Paper => self.opponent.clone(),
            // we need to win
            PRSMove::Scissors => self.opponent.get_winning(),
        };
        PRSRound {
            opponent: self.opponent.clone(),
            player: new_player,
        }
        .score()
    }
}

// parsing like there's no tomorrow
fn parse_player_move(input: &[u8]) -> IResult<&[u8], PRSMove> {
    alt((
        map(tag("X"), |_| PRSMove::Rock),
        map(tag("Y"), |_| PRSMove::Paper),
        map(tag("Z"), |_| PRSMove::Scissors),
    ))(input)
}

fn parse_opponent_move(input: &[u8]) -> IResult<&[u8], PRSMove> {
    alt((
        map(tag("A"), |_| PRSMove::Rock),
        map(tag("B"), |_| PRSMove::Paper),
        map(tag("C"), |_| PRSMove::Scissors),
    ))(input)
}

fn parse_game_line(i: &[u8]) -> IResult<&[u8], PRSRound> {
    let (input, (opponent_move, _, player_move)) =
        tuple((parse_opponent_move, tag(" "), parse_player_move))(i)?;
    Ok((
        input,
        PRSRound {
            opponent: opponent_move,
            player: player_move,
        },
    ))
}

fn parse_game(input: &[u8]) -> IResult<&[u8], Vec<PRSRound>> {
    let (input, game) = many0(terminated(parse_game_line, opt(line_ending)))(input)?;
    eof(input)?;
    return Ok((input, game));
}

#[test]
fn test_parse_game_line() {
    let input = b"A Z";
    let expected = Ok((
        &b""[..],
        PRSRound {
            opponent: PRSMove::Rock,
            player: PRSMove::Scissors,
        },
    ));
    assert_eq!(parse_game_line(input), expected);
}

#[test]
fn test_parse_game() {
    let input = r#"A Z
A X"#;
    let expected = Ok((
        &b""[..],
        vec![
            PRSRound {
                opponent: PRSMove::Rock,
                player: PRSMove::Scissors,
            },
            PRSRound {
                opponent: PRSMove::Rock,
                player: PRSMove::Rock,
            },
        ],
    ));
    assert_eq!(parse_game(input.as_bytes()), expected);
}

#[test]
fn test_parse_game_failed() {
    let junk = "junk";
    let input = r#"A Z
A Xjunk"#;
    let expected = Err(nom::Err::Error(nom::error::Error::new(
        junk.as_bytes(),
        nom::error::ErrorKind::Eof,
    )));
    assert_eq!(parse_game(input.as_bytes()), expected);
}

fn main() {
    let input = std::fs::read("./src/day2/input.txt").unwrap();
    let (_, game) = parse_game(&input).unwrap();

    // puzzle 1
    let score_strategy = game.iter().map(|round| round.score() as u32).sum::<u32>();
    println!("Score strategy: {}", score_strategy);

    // puzzle 2
    let score_strategy = game
        .iter()
        .map(|round| round.score_fixed() as u32)
        .sum::<u32>();
    println!("Score strategy fixed: {}", score_strategy);
}

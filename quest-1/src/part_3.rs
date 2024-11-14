use std::str::FromStr;
use winnow::combinator::{delimited, opt, repeat, terminated};
use winnow::token::{take, take_while};
use winnow::PResult;
use winnow::Parser;

mod part_2;
use part_2::map_potions;

const INPUT_P3: &str = include_str!("../../input/everybody_codes_e2024_q01_p3.txt");
fn main() {
    let num = parse_input(INPUT_P3);
    println!("Potion total: {}", num);
}

fn parse_input(s: &str) -> u32 {
    let (rem, num) = repeat(0.., parse_groups)
        .fold(
            || 0,
            |sum, next| sum + dbg!(next.parse::<Formation>().unwrap()).into_potions(),
        )
        .parse_peek(s)
        .unwrap();
    eprintln!("Rem: {}", rem);
    num
}

#[derive(Debug)]
enum Formation {
    Single(char),
    Pair(char, char),
    Three(char, char, char),
}
impl FromStr for Formation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(
            match (chars.next(), chars.next(), chars.next(), chars.next()) {
                (None, _, _, _) => Err(anyhow::Error::msg("Nothing in string."))?,
                (Some(a), None, _, _) => Formation::Single(a),
                (Some(a), Some(b), None, _) => Formation::Pair(a, b),
                (Some(a), Some(b), Some(c), None) => Formation::Three(a, b, c),
                (Some(_), Some(_), Some(_), Some(_)) => {
                    Err(anyhow::Error::msg("Unreasonable length."))?
                }
            },
        )
    }
}
impl Formation {
    fn into_potions(self) -> u32 {
        match self {
            Formation::Single(a) => map_potions(a),
            Formation::Pair(a, b) => 2 + map_potions(a) + map_potions(b),
            Formation::Three(a, b, c) => 6 + map_potions(a) + map_potions(b) + map_potions(c),
        }
    }
}

fn parse_empty<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., 'x').parse_next(input)
}

fn parse_single_group<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1..=3, 'A'..='D').parse_next(input)
}

fn parse_groups<'s>(input: &mut &'s str) -> PResult<&'s str> {
//    take(3usize).parse_next(input)
        delimited(opt(parse_empty), parse_single_group, opt(parse_empty)).parse_next(input)
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn sample_works() {
        //noinspection SpellCheckingInspection
        const SAMPLE: &str = "xBxAAABCDxCC";
        let num = parse_input(SAMPLE);
        assert_eq!(num, 30, "Should be equal according to the website");
    }
}

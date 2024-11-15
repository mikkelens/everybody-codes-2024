use std::collections::HashSet;

#[allow(unused_imports)]
use winnow::{
    ascii::*, combinator::*, prelude::*, stream::AsChar,
    token::*,
};

const INPUT_P2: &str = include_str!("../../input/everybody_codes_e2024_q02_p2.txt");
/// note: new thing is just the old thing but with unique symbols
fn main() {
    println!("Result: {}", parse_input(INPUT_P2).unwrap());
}

fn parse_input(mut s: &str) -> PResult<u32> {
    let (runes, inscription) = parse_parts.parse_next(&mut s)?;

    todo!()
}

fn parse_parts<'s>(input: &mut &'s str) -> PResult<(HashSet<&'s str>, Vec<&'s str>)> {
    separated_pair(parse_top, multispace1, parse_inscription).parse_next(input)
}

fn parse_top<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    preceded("WORDS:", parse_runes).parse_next(input)
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(0.., rest, ",").parse_next(input)
}

fn parse_inscription<'s>(input: &mut &'s str) -> PResult<Vec<&'s str>> {
    separated(0.., rest, multispace1).parse_next(input)
}

#[cfg(test)]
mod q2p2 {
    use crate::parse_input;

    #[test]
    fn sample_works() {
        //noinspection SpellCheckingInspection
        const SAMPLE: &str = {
            "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END"
        };
        assert_eq!(
            parse_input(SAMPLE).unwrap(),
            37,
            "There should be 37 runic symbols in total on \
        inscription"
        );
    }
}

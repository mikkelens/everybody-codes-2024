use std::collections::HashSet;

#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, prelude::*, stream::AsChar, token::*};

const INPUT_P2: &str = include_str!("../../input/everybody_codes_e2024_q02_p2.txt");
/// note: new thing is just the old thing but with unique symbols
fn main() {
    println!(
        "Result: {}",
        parse_input(INPUT_P2).expect("could not parse input")
    );
}

fn parse_input(mut s: &str) -> PResult<u32> {
    let (runes, inscription) = parse_parts.parse_next(&mut s)?;
    // note: inscription is a str of all the lines, no separation is explicit

    let mut total = 0;
    let mut frontier = 0;

    for i in 0..inscription.len() {
        for rune in runes.iter() {
            let rune_len = rune.len();
            if let Some(next_word) = inscription.get(i..(i + rune_len)) {
                if next_word == *rune || next_word.chars().rev().eq(rune.chars()) {
                    let offset = rune_len;
                    let next_frontier = frontier.max(i + offset);
                    let points = rune_len.min(next_frontier - frontier) as u32;
                    total += points;
                    #[cfg(debug_assertions)]
                    {
                        eprintln!("RUNE MATCH: {}", next_word);
                        eprintln!("- FRONTIER: {}", frontier);
                        eprintln!("- NEXT:     {} (i + {})", next_frontier, offset);
                        eprintln!("- POINTS:   {} (total: {})", points, total);
                        eprintln!();
                    }
                    frontier = next_frontier;
                }
            }
        }
    }

    Ok(total)
}

fn parse_parts<'s>(input: &mut &'s str) -> PResult<(HashSet<&'s str>, &'s str)> {
    separated_pair(parse_top, (line_ending, line_ending), rest).parse_next(input)
}
fn parse_top<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    let mut top = take_until(0.., "\n").parse_next(input)?;
    preceded("WORDS:", parse_runes).parse_next(&mut top)
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(1.., alpha1, ',').parse_next(input)
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
            parse_input(SAMPLE).expect("should be able to parse this"),
            37,
            "There should be 37 runic symbols in total on inscription"
        );
    }
}

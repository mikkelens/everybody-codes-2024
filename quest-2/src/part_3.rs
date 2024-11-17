use itertools::Itertools;
use std::collections::HashSet;
#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, prelude::*, stream::AsChar, token::*};

const INPUT_P3: &str = include_str!("../../input/everybody_codes_e2024_q02_p3.txt");
fn main() {
    println!(
        "Result: {}",
        parse_input(INPUT_P3).expect("could not parse")
    );
}

fn parse_input(mut s: &str) -> PResult<u32> {
    let (runes, armour) =
        separated_pair(parse_top, (line_ending, line_ending), parse_bottom).parse_next(&mut s)?;
    Ok(armour.all_runes_scale_locations(&runes))
}

fn parse_top<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    preceded("WORDS:", parse_runes).parse_next(input)
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(0.., alpha1, ',').parse_next(input)
}

fn parse_bottom<'s>(input: &mut &'s str) -> PResult<ScaleArmour> {
    todo!("Need to figure out how to parse and build a scale armour representation")
}

/// Some sort of scale location
type Loc = usize;
type Scale = char;
struct ScaleArmour(Vec<Scale>);
impl ScaleArmour {
    fn all_runes_scale_locations(&self, runes: &HashSet<&str>) -> u32 {
        runes
            .iter()
            .flat_map(|rune| self.rune_scale_locations(rune))
            .unique()
            .count() as u32
    }
    fn rune_scale_locations(&self, s: &str) -> HashSet<Loc> {
        todo!("Look through all directions for matches, then create set of them")
    }
}

#[cfg(test)]
mod q2p3 {
    use crate::parse_input;

    #[test]
    fn sample_works() {
        //noinspection SpellCheckingInspection
        const SAMPLE: &str = {
            "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
        };
        assert_eq!(
            parse_input(SAMPLE).expect("should be able to parse this"),
            10
        )
    }
}

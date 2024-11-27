use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use utils::debug_println;
#[allow(unused_imports)]
use winnow::{ascii::*, combinator::*, error::*, prelude::*, stream::AsChar, token::*};

const INPUT_P3: &str = include_str!("../../input/everybody_codes_e2024_q02_p3.txt");
fn main() {
    println!(
        "Result: {}",
        parse_input(INPUT_P3).expect("could not parse")
    );
}

fn parse_input(mut s: &str) -> PResult<u32> {
    let (runes, armour) = separated_pair(
        parse_top,
        (line_ending, line_ending)
            .context(StrContext::Expected(StrContextValue::StringLiteral("\n\n"))),
        parse_bottom,
    )
    .parse_next(&mut s)?;
    debug_println!("Runes: {:?}", runes);
    debug_println!("Armour: {}", armour);
    Ok(armour.all_runes_scale_locations(&runes))
}

fn parse_top<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    preceded("WORDS:", parse_runes).parse_next(input)
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(0.., parse_letters, ',').parse_next(input)
}
/// This is not the same thing as the `alpha1` parser, because this also includes non-ASCII letters
fn parse_letters<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., char::is_alphabetic).parse_next(input)
}

/// # Assumptions
/// This function assumes input is a rectangle of characters with no deviations, where the height
/// of the rectangle is defined by how many interspersed line endings there are within it.
/// # Failure
/// It will fail if passed an input string whose Unicode scalar characters do not form this shape.
fn parse_bottom(input: &mut &str) -> PResult<ScaleArmour> {
    let (scales, len) = separated(1.., parse_letters, line_ending)
        .verify_map(|lines: Vec<&str>| {
            if let Ok(len) = lines.iter().map(|s| s.chars().count()).all_equal_value() {
                Some((lines.into_iter().flat_map(|s| s.chars()).collect(), len))
            } else {
                None
            }
        })
        .context(StrContext::Expected(StrContextValue::Description(
            "all lines need to have an equal char count",
        )))
        .parse_next(input)?;
    Ok(ScaleArmour { scales, width: len })
}

/// Some sort of scale location
type Loc = usize;
/// An alphabetic symbol on the scale armour
type Scale = char;
struct ScaleArmour {
    scales: Vec<Scale>,
    width: usize,
}
impl ScaleArmour {
    // mostly certain this does what I want
    fn all_runes_scale_locations(&self, runes: &HashSet<&str>) -> u32 {
        runes
            .iter()
            .flat_map(|rune| self.rune_scale_locations(rune))
            .unique()
            .count() as u32
    }
    // this lets us pass the sample test, but fails at the larger input.
    // Maybe there is some edge case we're not handling? Or is the algorithm just right "on
    // accident" for the sample?
    fn rune_scale_locations(&self, rune_str: &str) -> HashSet<Loc> {
        let rune = &rune_str.chars().collect::<Vec<Scale>>()[..];
        debug_println!("SEARCHING: {} (len: {})", rune_str, rune.len());
        let all_scales = self.scales.iter().enumerate();

        let rows = all_scales.clone().chunks(self.width);
        let row_symbol_locations = rows.into_iter().flat_map(|row| {
            row.collect::<Vec<(_, _)>>()
                .into_iter()
                .cycle()
                .take(self.width + rune.len())
                .collect::<Vec<_>>()[..]
                .windows(rune.len())
                .filter_map(|word| {
                    let (locations, chars): (Vec<Loc>, Vec<char>) = word.iter().cloned().unzip();
                    if chars.eq(rune) || chars.iter().rev().eq(rune) {
                        Some(locations)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<HashSet<Loc>>()
        });

        let column_symbol_locations = all_scales
            .into_group_map_by(|(i, _)| i % self.width)
            .into_iter()
            .flat_map(|(_, column)| {
                column
                    .windows(rune.len())
                    .filter_map(|word| {
                        let (locations, chars): (Vec<_>, Vec<char>) = word.iter().cloned().unzip();
                        if chars.eq(rune) || chars.iter().rev().eq(rune) {
                            Some(locations)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect::<HashSet<Loc>>()
            });

        row_symbol_locations
            .merge(column_symbol_locations)
            .collect()
    }
}
impl Display for ScaleArmour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}x{} (WxH)",
            self.width,
            self.scales.len() / self.width
        )?;
        for row in self.scales.chunks(self.width) {
            for c in row {
                write!(f, "{}", c)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod q2p3 {
    use crate::parse_input;
    use utils::debug_println;

    #[test]
    fn sample_works() {
        //noinspection SpellCheckingInspection
        const SAMPLE: &str = {
            "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL"
        };
        debug_println!("Parsing sample:\n```\n{}\n```\n", SAMPLE);
        assert_eq!(
            parse_input(SAMPLE).expect("should be able to parse the sample"),
            10,
        )
    }

    #[test]
    fn custom_sample_works() {
        //noinspection SpellCheckingInspection
        const CUSTOM_SAMPLE: &str = {
            "WORDS:A,ABC,CD,ÅD

ABCDÅDCB
BAGHCBAC
ACGÅGÅCB"
        };
        debug_println!("Parsing sample:\n```\n{}\n```\n", CUSTOM_SAMPLE);
        assert_eq!(
            parse_input(CUSTOM_SAMPLE).expect("should be able to parse the sample"),
            3 + 1 + 1 + 1 + 2 + 3 + 3 + 3,
        )
    }

    #[ignore]
    #[test]
    fn answer_different() {
        // consider using snapshot testing instead e.g. the `insta` crate?
        assert_ne!(
            parse_input(crate::INPUT_P3).expect("should be able to parse the real thing"),
            5787 /* This number is incorrect, *including specifically* the first digit */
        );
    }
}
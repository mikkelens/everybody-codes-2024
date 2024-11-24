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
    let (runes, armour) =
        separated_pair(parse_top, (line_ending, line_ending), parse_bottom).parse_next(&mut s)?;
    debug_println!("Runes: {:?}", runes);
    debug_println!("Armour: {}", armour);
    Ok(armour.all_runes_scale_locations(&runes))
}

fn parse_top<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    preceded("WORDS:", parse_runes).parse_next(input)
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(0.., alpha1, ',').parse_next(input)
}

/// # Assumptions
/// This function assumes input is a rectangle of characters with no deviations, where the height
/// of the rectangle is defined by how many interspersed line endings there are within it.
/// # Failure
/// It will fail if passed an input string whose Unicode scalar characters do not form this shape.
fn parse_bottom(input: &mut &str) -> PResult<ScaleArmour> {
    let (scales, len) = separated(1.., alpha1, line_ending)
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

        let mut locations: HashSet<Loc> = HashSet::new();

        //        let rows = all_scales.clone().chunks(self.width);
        //        for row in rows.into_iter() {
        //            // note on wrapping behaviour: windows do not cycle past end
        //            let row = &row.into_iter().collect::<Vec<_>>()[..];
        //            // below slice wraps around enough so that windows are effectively wrapping
        //            let row_wrapping = &row
        //                .iter()
        //                .cycle()
        //                .take(row.len() + rune.len())
        //                .collect::<Vec<_>>()[..];
        //            for word in row_wrapping.windows(rune.len()) {
        //                let (word_locations, word_chars): (Vec<_>, Vec<char>) =
        //                    word.iter().cloned().cloned().unzip();
        //                if word_chars.eq(rune) || word_chars.iter().rev().eq(rune) {
        //                    locations.extend(word_locations);
        //                }
        //            }
        //        }

        //        let columns = all_scales
        //            .clone()
        //            .into_group_map_by(|(i, _)| i % self.width);
        //        debug_assert_eq!(
        //            columns.len(),
        //            self.width,
        //            "Column count must add up to agreed amount",
        //        );
        //        debug_assert!(
        //            columns.values().map(|column| column.len()).all_equal(),
        //            "Length of columns must all be equal"
        //        );
        //        for (_offset_key, column) in columns.into_iter() {
        //            // note on wrapping behaviour: windows do not cycle past end
        //            // below slice wraps around enough so that windows are effectively wrapping
        //            for word in column.windows(rune.len()) {
        //                let (word_locations, word_chars): (Vec<_>, Vec<char>) =
        //                    word.iter().cloned().unzip();
        //                if word_chars.eq(rune) || word_chars.iter().rev().eq(rune) {
        //                    debug_println!("Matched, adding locations: {:?}", word_locations);
        //                    locations.extend(word_locations);
        //                }
        //            }
        //        }

        let row_iter = all_scales
            .clone()
            .chunks(self.width)
            .into_iter()
            .flat_map(|row| {
                row.collect::<Vec<(_, _)>>()
                    .into_iter()
                    .cycle()
                    .take(self.width + rune.len())
                    .collect::<Vec<_>>()[..]
                    .windows(rune.len())
                    .filter_map(|word| {
                        let (locations, chars): (Vec<Loc>, Vec<char>) =
                            word.iter().cloned().unzip();
                        if chars.eq(rune) || chars.iter().rev().eq(rune) {
                            Some(locations)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect::<HashSet<Loc>>()
            });

        let column_iter = all_scales
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

        row_iter.merge(column_iter).collect()
    }
}
impl Display for ScaleArmour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}(w) times {}(h):",
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

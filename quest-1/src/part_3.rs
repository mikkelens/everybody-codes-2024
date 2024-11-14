use itertools::Itertools;
use std::str::FromStr;
use winnow::combinator::{delimited, opt, repeat, terminated};
use winnow::token::{take, take_while};
use winnow::PResult;
use winnow::Parser;

mod part_2;
use part_2::map_potion;

const INPUT_P3: &str = include_str!("../../input/everybody_codes_e2024_q01_p3.txt");
fn main() {
    let num = parse_input(INPUT_P3);
    println!("Potion total: {}", num);
}

/// yes this really is how you're supposed to do it. delimiting and proper group parsing gives
/// the wrong result, despite what you might infer from the text.
/// "AxBx" is a group of two (A & B), *not* two separate attackers, because of *how they're placed*
///
/// See below commit for a more detail-correct parser:
/// ```
/// e3a546c245f7ed99ccee6a4028688457d448cb81
/// ```
fn parse_input(s: &str) -> u32 {
    s.chars().tuples().fold(0, |mut sum, (a, b, c)| {
        let mut count = 0;
        if let Some(a) = try_battle(a) {
            sum += a;
            count += 1;
        }
        if let Some(b) = try_battle(b) {
            sum += b;
            count += 1;
        }
        if let Some(c) = try_battle(c) {
            sum += c;
            count += 1;
        }
        match count {
            2 => sum += 2,
            3 => sum += 6,
            _ => {}
        }
        sum
    })
}

fn try_battle(c: char) -> Option<u32> {
    if c == 'x' {
        None
    } else {
        Some(map_potion(c))
    }
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

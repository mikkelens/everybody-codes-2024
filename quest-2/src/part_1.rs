use std::collections::HashSet;
use winnow::combinator::{preceded, separated};
use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::token::{take_till, take_while};

const INPUT_P1: &str = include_str!("../../input/everybody_codes_e2024_q02_p1.txt");
fn main() {
    println!("Result: {}", parse_input(INPUT_P1).unwrap());
}

fn parse_input(mut input: &str) -> PResult<u32> {
    let mut top = Parser::<&str, &str, _>::parse_next(
        &mut take_while(0.., |a| !winnow::stream::AsChar::is_newline(a)),
        &mut input,
    )?;
    let bottom = input;
    dbg!(top, bottom);
    let runes = parse_runes.parse_next(&mut top)?;
    let inscription = bottom;
    Ok(count_runic_in_inscription(inscription, &runes))
}
fn parse_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    eprintln!("Attempting to parse runes: {:?}", input);
    preceded("WORDS:", separate_runes)
        .context(StrContext::Label("some `WORDS:` in front of runes"))
        .parse_next(input)
}
fn separate_runes<'s>(input: &mut &'s str) -> PResult<HashSet<&'s str>> {
    separated(
        0..,
        take_till(0.., ',').context(StrContext::Label("taking characters between `,`s")),
        ',',
    )
    .context(StrContext::Label("runes should be separated by `,`s"))
    .parse_next(input)
}

/// assume words do not fully overlap each other, and that runes do not contain any spaces
fn count_runic_in_inscription(inscription: &str, runes: &HashSet<&str>) -> u32 {
    let mut sum: u32 = 0;
    for word in inscription.split(' ') {
        sum += runes
            .iter()
            .filter(|&rune| {
                let rune_present = word.contains(rune);
                if rune_present {
                    eprintln!("Rune {} present in {}", rune, word);
                }
                rune_present
            })
            .count() as u32;
    }
    sum
}

#[cfg(test)]
mod q2p1 {
    use crate::{count_runic_in_inscription, parse_input, parse_runes};

    #[test]
    fn raw_sample_works() {
        const RAW_SAMPLE: &str = r"WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        assert_eq!(parse_input(RAW_SAMPLE).unwrap(), 4);
    }

    #[test]
    fn samples_all_work() {
        const SAMPLE_RUNES: &str = "WORDS:THE,OWE,MES,ROD,HER";
        //noinspection SpellCheckingInspection
        const SAMPLE_INSCRIPTIONS: [(&str, u32); 4] = [
            ("AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE", 4),
            ("THE FLAME SHIELDED THE HEART OF THE KINGS", 3),
            ("POWE PO WER P OWE R", 2),
            ("THERE IS THE END", 3),
        ];

        let mut sample_runic = SAMPLE_RUNES;
        let runes = parse_runes(&mut sample_runic).unwrap();
        eprintln!("Runes: ({})\n{:?} ({})", SAMPLE_RUNES, runes, runes.len());
        for (inscription, runic_word_count) in SAMPLE_INSCRIPTIONS {
            eprintln!("Counting runes in inscription: {}", inscription);
            let count = count_runic_in_inscription(inscription, &runes);
            eprintln!("There are {} runic words in inscription!", count);
            assert_eq!(count, runic_word_count);
            eprintln!()
        }
    }
}

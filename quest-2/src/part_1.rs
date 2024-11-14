use std::collections::HashSet;

const INPUT_P1: &str = include_str!("../../input/everybody_codes_e2024_q02_p1.txt");
fn main() {
    println!("Result: {}", parse_input(INPUT_P1));
}

fn parse_input(input: &str) -> u32 {
    todo!()
}
fn parse_runic_set(s: &str) -> HashSet<&str> {
    todo!()
}
fn count_present<'a>(inscription: &str, set: &HashSet<&str>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{count_present, parse_runic_set};

    const SAMPLE_RUNIC_SET: &str = "WORDS:THE,OWE,MES,ROD,HER";
    //noinspection SpellCheckingInspection
    const SAMPLE_INSCRIPTIONS: [(&str, u32); 4] = [
        (
            "AWAKEN THE POWER ADORNED WITH THE FLAMES \
    BRIGHT IRE",
            4,
        ),
        ("THE FLAME SHIELDED THE HEART OF THE KINGS", 3),
        ("POWE PO WER P OWE R", 2),
        ("THERE IS THE END", 3),
    ];
    #[test]
    fn sample_works() {
        let set = parse_runic_set(SAMPLE_RUNIC_SET);
        for (inscription, runic_word_count) in SAMPLE_INSCRIPTIONS {
            assert_eq!(count_present(inscription, &set), runic_word_count);
        }
    }
}

use itertools::Itertools;

const INPUT_P2: &str = include_str!("../../input/everybody_codes_e2024_q01_p2.txt");
#[allow(unused)]
fn main() {
    let potion_sum: u32 = INPUT_P2.chars().tuples().map(map_pair).sum();
    println!("Potion sum: {}", potion_sum);
}

fn map_pair(pair: (char, char)) -> u32 {
    match pair {
        ('x', 'x') => 0,
        ('x', single) | (single, 'x') => map_potions(single),
        (a, b) => 2 + map_potions(a) + map_potions(b),
    }
}

pub fn map_potions(c: char) -> u32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        unknown => unimplemented!("unknown char: {}", unknown),
    }
}

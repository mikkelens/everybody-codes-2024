const INPUT_P1: &str = include_str!("../../input/everybody_codes_e2024_q01_p1.txt");
fn main() {
    let potion_sum: u32 = INPUT_P1.chars().map(map_potion).sum();
    println!("Potion sum: {}", potion_sum);
}

fn map_potion(c: char) -> u32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        unknown => unimplemented!("unknown char: {}", unknown),
    }
}

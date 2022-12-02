use std::io;
use phf::{phf_map};

static RPS: phf::Map<char, u8> = phf_map! {
    'A' => 1,
    'B' => 2,
    'C' => 3,
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
};


fn get_points(a: u8, result: u8) -> u8 {
    // lose
    if result == 1 {
        // 1 -> 3
        // 2 -> 1
        // 3 -> 2

        // +3 to make sure we don't go below 0 (u8)
        // -1 to convert to 0-based index
        // -1 to get "left" (losing) shape
        // %3 to be within our rock-paper-scissors ring
        // +1 restores 1-based indexing
        // +0 is the "award" when you lose
        return (a + 3 - 1 - 1) % 3 + 1 + 0;
    }

    // win
    if result == 3 {
        // 1 -> 2
        // 2 -> 3
        // 3 -> 1
        // same idea as in "lose" branch but:
        // +1 to get "right" (winning) shape
        // no +3 because we always stay within [0;3]
        // +6 is a reward when you win
        return (a + 1 - 1) % 3 + 1 + 6;
    }

    // draw
    // return the same shape as the opponent (= a)
    // +3 reward for the draw
    return a + 3;
}


fn main() {
    let mut total_points: u32 = 0;

    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.is_empty() {
            break;
        }

        let c0 = line.chars().nth(0).unwrap();
        let c2 = line.chars().nth(2).unwrap();
        // println!("{} {}", c0, RPS[&c0]);
        // println!("{} {}", c2, RPS[&c2]);

        let points = u32::from(get_points(RPS[&c0], RPS[&c2]));
        total_points = total_points + points;
    }

    println!("The answer is {total_points}");
}

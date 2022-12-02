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


fn get_points(a: u8, b: u8) -> u32 {
    // by default, you get the value of your shape
    let score: u32 = u32::from(b);

    // draw
    if a == b {
        return score + 3;
    }

    // winning combinations
    // 1 2
    // 2 3
    // 3 1
    if (a + 1) % 3 == b % 3 {
        return score + 6;
    }

    return score;
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

        let points = get_points(RPS[&c0], RPS[&c2]);
        total_points = total_points + points;
    }

    println!("The answer is {total_points}");
}

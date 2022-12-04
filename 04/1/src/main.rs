use std::io;
use scanf::sscanf;


fn main() {
    let mut line: String;
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    let mut c: u8 = 0;
    let mut d: u8 = 0;
    let mut contains: u32 = 0;
    let mut overlaps: u32 = 0;

    loop {
        line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }

        if sscanf!(&line, "{}-{},{}-{}", a, b, c, d).is_ok() {
            // println!("[{};{}]  [{};{}]", a, b, c, d);
            if (c <= a && a <= d && c <= b && b <= d) || (a <= c && c <= b && a <= d && d <= b) {
                // println!("contains!");
                contains += 1;
            }
            if (c <= a && a <= d) || (c <= b && b <= d) || (a <= c && c <= b) || (a <= d && d <= b) {
                // println!("overlaps!");
                overlaps += 1;
            }
        }
    }

    println!("Contains: {contains}");
    println!("Overlaps: {overlaps}");
}

use std::env;
use std::io;

fn main() {
    let mut start_of_packet_length: usize = 4;
    if env::args().len() > 1 {
        start_of_packet_length = env::args().nth(1).unwrap().parse().unwrap();
    }

    let mut line: String;

    line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let bytes = line.as_bytes();

    let mut pos = start_of_packet_length - 1;
    while pos < line.len() {
        // 1st naive implementation
        // if bytes[pos-3] != bytes[pos-2] && bytes[pos-3] != bytes[pos-1] && bytes[pos-3] != bytes[pos] &&
        //    bytes[pos-2] != bytes[pos-1] && bytes[pos-2] != bytes[pos] &&
        //    bytes[pos-1] != bytes[pos] {
        //         break
        //     }

        // general solution is O(N^3) but it works ¯\_(ツ)_/¯
        let mut is_marker: bool = true;
        for i in 0..start_of_packet_length {
            for j in 0..i {
                if bytes[pos-usize::from(i)] == bytes[pos-usize::from(j)] {
                    is_marker = false;
                    break;
                }
            }
            if !is_marker {
                break;
            }
        }

        if is_marker {
            break;
        }

        pos += 1;
    }

    pos += 1;
    println!("Answer: {pos}");
}

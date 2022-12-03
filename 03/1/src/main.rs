use std::io;


fn get_priority(c: char) -> u8 {
    let ascii: u8 = c as u8;

    // 'a' > 'A'
    if ascii >= 'a' as u8 {
        return ascii - 'a' as u8 + 1;
    }
    return ascii - 'A' as u8 + 26 + 1;
}


fn main() {
    // let's waste index 0 and use 1-indexing to avoid bugs :)
    let mut left_compartment: [u8; 1 + 52];

    let mut sum_priorities: u32 = 0;

    loop {
        let mut rucksack = String::new();

        // make sure we start from scratch for each rucksack
        left_compartment = [0; 1 + 52];

        io::stdin()
            .read_line(&mut rucksack)
            .expect("Failed to read line");

        if rucksack.is_empty() {
            break;
        }

        let len = rucksack.len();  // including newline but integer /2 yields correct result
        // TODO: better way? slicing?
        let left: String = rucksack.chars().skip(0).take(len/2).collect();
        let right: String = rucksack.chars().skip(len/2).take(len/2).collect();

        // println!("len = {}", len);
        // println!("left = {}", left);
        // println!("right = {}", right);

        let mut n: u8;
        for c in left.chars() {
            n = get_priority(c);
            // println!("{} {}", c, n);
            left_compartment[usize::from(n)] = 1;
        }

        // for x in left_compartment {
        //     print!("{x} ");
        // }

        for c in right.chars() {
            n = get_priority(c);
            // println!("right {} = {}", c, n);
            if left_compartment[usize::from(n)] == 1 {
                // println!("found match: {} {}", c, n);
                sum_priorities += u32::from(n);
                break;
            }
        }
    }

    println!("The answer is {sum_priorities}");
}

use std::io;


fn get_priority(c: char) -> u8 {
    let ascii: u8 = c as u8;

    // 'a' > 'A'
    if ascii >= 'a' as u8 {
        return ascii - 'a' as u8 + 1;
    }
    return ascii - 'A' as u8 + 26 + 1;
}


fn count_items(rucksack: String, items: &mut [u8; 53]) {
    let mut n: u8;
    for c in rucksack.chars() {
        n = get_priority(c);
        items[usize::from(n)] += 1;
    }
}


fn read_rucksack() -> String {
    let mut rucksack: String = String::new();
    io::stdin()
        .read_line(&mut rucksack)
        .expect("Failed to read line");

    rucksack.pop();  // let's hope it removes the newline
    // println!("rucksack: {}", rucksack);

    return rucksack;
}


fn main() {
    // let's waste index 0 and use 1-indexing to avoid bugs :)
    let mut items_1: [u8; 1 + 52];
    let mut items_2: [u8; 1 + 52];

    let mut rucksack: String;
    let mut sum_priorities: u32 = 0;

    loop {
        // make sure we start from scratch for each rucksack, fill with zeros
        items_1 = [0; 1 + 52];
        items_2 = [0; 1 + 52];

        /* rucksack #1 */
        rucksack = read_rucksack();
        if rucksack.is_empty() {
            break;
        }
        count_items(rucksack, &mut items_1);

        /* rucksack #2 */
        rucksack = read_rucksack();
        count_items(rucksack, &mut items_2);

        /* rucksack #3 */
        rucksack = read_rucksack();

        let mut n: u8;
        let mut i: usize;
        for c in rucksack.chars() {
            n = get_priority(c);
            i = usize::from(n);
            if items_1[i] > 0 && items_2[i] > 0 {
                sum_priorities += u32::from(n);
                break;
            }
        }
    }

    println!("The answer is {sum_priorities}");
}

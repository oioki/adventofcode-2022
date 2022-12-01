use std::io;

fn main() {
    let mut cur_sum: u32 = 0;
    let mut max_sum: u32 = 0;
    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        if num == "\n" || num.is_empty() {
            if cur_sum > max_sum {
                max_sum = cur_sum;
            }

            if num.is_empty() {
                break;
            }

            cur_sum = 0;
            continue;
        }

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        cur_sum = cur_sum + num;
    }

    println!("The answer is {max_sum}");
}

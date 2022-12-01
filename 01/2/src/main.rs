use std::io;

fn main() {
    let mut cur_sum: u32 = 0;
    // let mut max_sum: u32 = 0;

    let mut top_1_sum: u32 = 0;
    let mut top_2_sum: u32 = 0;
    let mut top_3_sum: u32 = 0;

    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        if num == "\n" || num.is_empty() {
            if cur_sum > top_1_sum {
                top_3_sum = top_2_sum;
                top_2_sum = top_1_sum;
                top_1_sum = cur_sum;
            }
            else if cur_sum > top_2_sum {
                top_3_sum = top_2_sum;
                top_2_sum = cur_sum;
            }
            else if cur_sum > top_3_sum {
                top_3_sum = cur_sum;
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

    let total_calories: u32 = top_1_sum + top_2_sum + top_3_sum;
    println!("The answer is {total_calories}");
}

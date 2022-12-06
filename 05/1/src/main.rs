use std::env;
use std::io;
use scanf::sscanf;
use std::collections::LinkedList;

const MAX_STACKS: usize = 9;

#[derive(PartialEq)]
enum CraneMode {
    CrateMover9000,
    CrateMover9001,
}

fn print_stacks(stacks: &[std::collections::LinkedList<char>], num_stacks: usize) {
    let mut i = 0;
    while i < num_stacks {
        print!("Stack #{}: ", i + 1);

        for element in stacks[i].iter() {
            print!("{}", element);
        }
        println!();

        i += 1;
    }
}


fn main() {
    let mut mode = CraneMode::CrateMover9000;
    if env::args().len() > 1 {
        if env::args().nth(1).unwrap() == "9001" {
            mode = CraneMode::CrateMover9001;
        }
    }

    let mut line: String;

    let mut stack = vec![LinkedList::<char>::new(); MAX_STACKS];
    let mut num_stacks = 0;

    // only used in Crate Mover 9001
    let mut temp_stack = LinkedList::<char>::new();

    loop {
        line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line == "\n" {
            break;
        }

        // println!("len: {}", line.len());
        num_stacks = line.len() / 4;
        // println!("number of stacks: {}", num_stacks);

        let mut i_stack = 0;
        let mut i = 1;
        let mut c: char;
        while i < line.len() {
            c = line.chars().nth(i).unwrap();
            // println!("c: {}", c);
            if 'A' <= c && c <= 'Z' {
                stack[i_stack].push_front(c);
            }
            i_stack += 1;
            i += 4;
        }
    }

    print_stacks(&stack, num_stacks);

    loop {
        line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }

        let mut count: u8 = 0;
        let mut from: usize = 0;
        let mut to: usize = 0;

        // Example: "move 1 from 2 to 1"
        if sscanf!(&line, "move {} from {} to {}", count, from, to).is_ok() {
            if mode == CraneMode::CrateMover9000 {
                // println!("{} blocks, {} -> {}", count, from, to);
                for _i in 0..count {
                    let mut _crate = stack[from-1].pop_back().unwrap();  // "crate" is the reserved word in Rust, lol
                    // println!("crate = {:?}", _crate);
                    stack[to-1].push_back(_crate);
                }
            }

            if mode == CraneMode::CrateMover9001 {
                for _i in 0..count {
                    let mut _crate = stack[from-1].pop_back().unwrap();  // "crate" is the reserved word in Rust, lol
                    // println!("crate = {:?}", _crate);
                    temp_stack.push_back(_crate);
                }
                for _i in 0..count {
                    let mut _crate = temp_stack.pop_back().unwrap();  // "crate" is the reserved word in Rust, lol
                    // println!("crate = {:?}", _crate);
                    stack[to-1].push_back(_crate);
                }
            }

            // print_stacks(&stack, num_stacks);
        }
    }

    let mut result: String = String::new();
    let mut i = 0;
    while i < num_stacks {
        let top_crate = stack[i].pop_back().unwrap();
        // println!("top_crate = {}", top_crate);
        result.push(top_crate);
        i += 1;
    }

    println!("Answer: {result}");
}

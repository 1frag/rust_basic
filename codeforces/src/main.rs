use std::io;
use std::env;

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn read_vec_i32() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input
        .split(" ")
        .map(|s| s.trim().parse().unwrap())
        .collect();
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

// 1455A
pub mod t_strange_functions {
    use crate::{read_i32, read_string};

    pub fn solve() {
        let n = read_i32();
        for _ in 0..n {
            println!("{}", read_string().len());
        }
    }
}

// 1455B
pub mod t_jumps {
    use crate::{read_i32};

    fn solve_one(x: i32) -> i32 {
        let mut left = 1;
        let mut right = 44720;
        let mut last_success = 1;
        while left <= right {
            let mid = (left + right) >> 1;
            let sum = (mid * mid + mid) >> 1;
            if sum < x { left = mid + 1 };
            if sum > x {
                right = mid - 1;
                last_success = mid
            };
            if sum == x { return mid; };
        }
        let sum = (last_success * last_success + last_success) >> 1;
        if sum == x + 1 { return last_success + 1; }
        return last_success;
    }

    pub fn solve() {
        let t = read_i32();
        for _ in 0..t {
            let x = read_i32();
            println!("{}", solve_one(x));
        }
    }
}

// 1455C
pub mod t_ping_pong {
    use crate::{read_i32, read_vec_i32};

    fn solve_one(a: i32, b: i32) -> (i32, i32) {
        if a == 0 { return (0, b) }
        if a == 1 && b != 0 { return (0, b) }
        if b == 0 { return (a, 0) }
        return (a - 1, b);
    }

    pub fn solve() {
        let t = read_i32();
        for _ in 0..t {
            let inp = read_vec_i32();
            let (a, b) = self::solve_one(inp[0], inp[1]);
            println!("{} {}", a, b);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Unsupported filter for problem");
        return;
    }
    let task_name = args[1].to_string();

    if task_name == "1455A" {
        return t_strange_functions::solve();
    } else if task_name == "1455B" {
        return t_jumps::solve();
    } else if task_name == "1455C" {
        return t_ping_pong::solve();
    } else {
        println!("Problem not found");
    }
}

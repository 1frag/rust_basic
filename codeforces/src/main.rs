use std::io;
use std::env;

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

pub mod t_strange_functions { // http://codeforces.com/problemset/problem/1455/A
    use crate::{read_i32, read_string};

    pub fn solve() {
        let n = read_i32();
        for _ in 0..n {
            println!("{}", read_string().len());
        }
    }
}

pub mod t_jumps { // http://codeforces.com/contest/1455/problem/B
    use crate::{read_i32};

    fn solve_one(x: i32) -> i32 {
        let mut left = 1;
        let mut right = 44720;
        let mut last_success = 1;
        while left <= right {
            let mid = (left + right) >> 1;
            let sum = (mid * mid + mid) >> 1;
            if sum < x { left = mid + 1 };
            if sum > x { right = mid - 1; last_success = mid };
            if sum == x { return mid };
        }
        let sum = (last_success * last_success + last_success) >> 1;
        if sum == x + 1 { return last_success + 1 }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Unsupported filter for problem");
        return;
    }
    let task_name = args[1].to_string();

    if task_name == "1455/A" {
        return t_strange_functions::solve();
    } else if task_name == "1455/B" {
        return t_jumps::solve();
    } else {
        println!("Problem not found");
    }
}

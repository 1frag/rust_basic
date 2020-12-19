use std::env;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;

fn read_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn read_vec_ixx<T>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
    T: FromStr,
{
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
    use crate::read_i32;

    fn solve_one(x: i32) -> i32 {
        let mut left = 1;
        let mut right = 44720;
        let mut last_success = 1;
        while left <= right {
            let mid = (left + right) >> 1;
            let sum = (mid * mid + mid) >> 1;
            if sum < x {
                left = mid + 1
            };
            if sum > x {
                right = mid - 1;
                last_success = mid
            };
            if sum == x {
                return mid;
            };
        }
        let sum = (last_success * last_success + last_success) >> 1;
        if sum == x + 1 {
            return last_success + 1;
        }
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
    use crate::{read_i32, read_vec_ixx};

    fn solve_one(a: i32, b: i32) -> (i32, i32) {
        if a == 0 {
            return (0, b);
        }
        if a == 1 && b != 0 {
            return (0, b);
        }
        if b == 0 {
            return (a, 0);
        }
        return (a - 1, b);
    }

    pub fn solve() {
        let t = read_i32();
        for _ in 0..t {
            let inp = read_vec_ixx();
            let (a, b) = self::solve_one(inp[0], inp[1]);
            println!("{} {}", a, b);
        }
    }
}

fn vec_is_sorted(a: &Vec<i32>) -> bool {
    for i in 1..(a.len()) {
        if a[i] < a[i - 1] {
            return false;
        }
    }
    true
}

fn swap_value<'a, T: Clone>(a: &'a mut T, b: &'a mut T) {
    let tmp = a.clone();
    *a = b.clone();
    *b = tmp.clone();
}

// 1455D
pub mod sequence_and_swaps {
    use crate::{read_i32, read_vec_ixx, swap_value, vec_is_sorted};

    fn solve_one() -> i32 {
        let (mut v, mut a) = (read_vec_ixx(), read_vec_ixx());
        let (mut sm, mut cnt) = (0, 0);
        for i in (1..v[0]).map(|i| i as usize) {
            sm += (a[i - 1] > a[i]) as i32;
        }

        let n = (v[0] - 1) as usize;
        for i in (0..v[0]).map(|i| i as usize) {
            if sm == 0 {
                break;
            }
            if a[i] > v[1] {
                if i > 0 && ((a[i - 1] > a[i]) && (a[i - 1] <= v[1])) {
                    sm -= 1
                };
                if i > 0 && ((a[i - 1] <= a[i]) && (a[i - 1] > v[1])) {
                    sm += 1
                };
                if i < n && ((a[i] > a[i + 1]) && (v[1] <= a[i + 1])) {
                    sm -= 1
                };
                if i < n && ((a[i] <= a[i + 1]) && (v[1] > a[i + 1])) {
                    sm += 1
                };

                swap_value(&mut v[1], &mut a[i]);
                cnt += 1;
            }
        }
        if vec_is_sorted(&a) {
            return cnt;
        }
        return -1;
    }

    pub fn solve() {
        for _ in 0..(read_i32()) {
            println!("{}", self::solve_one());
        }
    }
}

fn calculate_presum<'a>(vec: &'a Vec<i64>) -> impl Iterator<Item = i64> + 'a {
    let mut last = 0_i64;
    (0..1).chain((*vec).iter().map(move |s| {
        last += s;
        last.clone()
    }))
}

// 231C
pub mod to_add_or_not_to_add {
    use crate::{calculate_presum, read_vec_ixx};

    pub fn solve() {
        let v: Vec<i64> = read_vec_ixx();
        let k = v[1];
        let mut a: Vec<i64> = read_vec_ixx();
        a.sort();
        let presum: Vec<i64> = calculate_presum(&a).collect();
        let mut best = (0, 0);
        for i in 0..(a.len()) {
            if (i + 1 != a.len()) && (a[i + 1] == a[i]) {
                continue;
            }
            let (mut left, mut right, mut last_success) = (0, i as i64, 0);
            while left <= right {
                let mid: i64 = (right + left) / 2;
                let sm: i64 = presum[i + 1] - presum[mid as usize];
                let m = (i as i64) - mid + 1;
                if m * a[i] - sm > k {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                    last_success = m;
                }
            }
            if best.0 < last_success {
                best.0 = last_success;
                best.1 = a[i].clone();
            }
        }
        println!("{} {}", best.0, best.1);
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
    } else if task_name == "1455D" {
        return sequence_and_swaps::solve();
    } else if task_name == "231C" {
        return to_add_or_not_to_add::solve();
    } else {
        println!("Problem not found");
    }
}

#[cfg(test)]
mod tests {
    use crate::swap_value;

    #[test]
    fn swap_value_test() {
        let (mut a, mut b) = (5, 6);

        swap_value(&mut a, &mut b);
        assert_eq!(a, 6);
        assert_eq!(b, 5);
    }
}

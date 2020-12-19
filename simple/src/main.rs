use rand::Rng;
use std::collections::HashSet;
use std::env;
use std::io;
use std::process::{exit, Command};

fn cmd_hello() -> String {
    return "Hello, world!".to_string();
}

fn cmd_sum(args: Vec<String>) -> String {
    let mut res = 0;
    for i in 2..args.len() {
        match args[i].parse::<i32>() {
            Ok(n) => res += n,
            Err(e) => {
                println!("{} couldn't be parsed ({})", args[i], e);
            }
        }
    }
    return format!("Sum of given numbers is {}", res);
}

fn cmd_dirs() -> Vec<String> {
    fn listdir() -> Vec<String> {
        let vec_utf8output = Command::new("sh")
            .args(&["-c", "find -maxdepth 1 -type d"])
            .output()
            .unwrap()
            .stdout;

        let ls_output_or_none: Option<String> = match String::from_utf8(vec_utf8output) {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        if ls_output_or_none.is_none() {
            exit(1)
        };
        let ls_output = ls_output_or_none.unwrap();
        let x: Vec<&str> = ls_output.split("\n").collect();
        let v2: Vec<String> = x.iter().map(|s| s.to_string()).collect();
        return v2;
    }

    let lst = listdir();
    let mut result: Vec<String> = Vec::new();
    println!(
        "{}",
        match lst.len() - 2 {
            0 => "There is no directories here:".to_string(),
            1 => "There is only directory here:".to_string(),
            _ => format!("There are {} directories here:", lst.len() - 2).to_string(),
        }
    );
    for dir_name in lst {
        if dir_name == "." || dir_name == "" {
            continue;
        }
        result.push(dir_name[2..].to_string());
    }
    result.sort();
    return result;
}

enum AskAnswer {
    Start,
    Equals,
    Less,
    Greater,
}

fn user_asker(ans: AskAnswer) -> Option<i32> {
    match ans {
        AskAnswer::Start => {
            println!("Try to guess the number!")
        }
        AskAnswer::Equals => {
            println!("You are right!");
            return None;
        }
        AskAnswer::Less => {
            println!("The secret number should be less");
        }
        AskAnswer::Greater => {
            println!("The secret number should be greater");
        }
    }

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        guess = guess.strip_suffix("\n").unwrap().to_string();
        match guess.parse::<i32>() {
            Ok(numb) => {
                return Some(numb);
            }
            Err(e) => {
                println!("Please, input valid number [{}]", e);
            }
        }
    }
}

fn cmd_guess(mut ask: impl FnMut(AskAnswer) -> Option<i32>) -> String {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut first = true;
    let mut last_numb = None;
    loop {
        if first {
            last_numb = ask(AskAnswer::Start);
            first = false;
        }
        if last_numb.is_none() {
            return "".to_string();
        };

        if last_numb.unwrap() == secret_number {
            last_numb = ask(AskAnswer::Equals);
        } else if last_numb.unwrap() < secret_number {
            last_numb = ask(AskAnswer::Greater);
        } else if last_numb.unwrap() > secret_number {
            last_numb = ask(AskAnswer::Less);
        }
    }
}

fn cmd_primes(n: i32) -> String {
    let mut complex: HashSet<i32> = HashSet::new();
    for i in 2..n {
        if complex.contains(&i) {
            continue;
        };
        for j in (i * i..n).step_by(i as usize) {
            complex.insert(j);
        }
    }
    let mut primes: HashSet<i32> = HashSet::new();
    primes.extend(2..n);
    let mut v = primes.difference(&complex).into_iter().collect::<Vec<_>>();
    v.sort();
    return format!("{:?}", v);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "hello" {
        println!("{}", cmd_hello());
    } else if args[1] == "sum" {
        println!("{}", cmd_sum(args));
    } else if args[1] == "dirs" {
        println!("{:?}", cmd_dirs());
    } else if args[1] == "guess" {
        println!("{}", cmd_guess(user_asker));
    } else if args[1] == "primes" {
        println!(
            "{}",
            cmd_primes(if args.len() == 3 {
                args[2].parse().unwrap()
            } else {
                100
            })
        );
    } else {
        println!("undefined action for args[0]={}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd_dirs, cmd_guess, cmd_hello, cmd_primes, cmd_sum, AskAnswer};

    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn hello_world_test() {
        assert_eq!(cmd_hello(), "Hello, world!");
    }

    #[test]
    fn sum_test() {
        assert_eq!(
            cmd_sum(vec!["1".to_string(), "sum".to_string(), "3".to_string()]),
            "Sum of given numbers is 3"
        );
    }

    #[test]
    fn dirs_test() {
        assert_eq!(cmd_dirs(), ["src"]);
    }

    #[test]
    fn guess_test() {
        let mut correct = false;
        let mut left = 1;
        let mut right = 101;
        let mut last_asked: Option<i32> = None;

        let test_asker = |ans: AskAnswer| -> Option<i32> {
            match ans {
                AskAnswer::Equals => {
                    correct = true;
                    return None;
                }
                AskAnswer::Less => right = last_asked.unwrap() - 1,
                AskAnswer::Greater => left = last_asked.unwrap() + 1,
                _ => {}
            };
            last_asked = Some((left + right) / 2);
            if left <= right {
                return last_asked;
            }
            return None;
        };
        cmd_guess(test_asker);
        assert_eq!(correct, true);
    }

    #[test]
    fn primes_test() {
        assert_eq!(
            cmd_primes(42),
            "[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]"
        );
    }
}

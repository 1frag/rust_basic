use std::env;
use std::process::{Command, exit};

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

fn cmd_dirs() -> String {
    fn listdir() -> Vec<String> {
        let vec_utf8output = Command::new("sh")
            .args(&["-c", "find -maxdepth 1 -type d"]).output().unwrap().stdout;

        let ls_output_or_none: Option<String> = match String::from_utf8(vec_utf8output) {
            Ok(path) => Some(path),
            Err(_) => None,
        };
        if ls_output_or_none.is_none() { exit(1) };
        let ls_output = ls_output_or_none.unwrap();
        let x: Vec<&str> = ls_output.split("\n").collect();
        let v2: Vec<String> = x.iter().map(|s| { s.to_string() }).collect();
        return v2;
    }

    let lst = listdir();
    let mut result: String = match lst.len() - 2 {
        0 => "There is no directories here:".to_string(),
        1 => "There is only directory here:".to_string(),
        _ => format!("There are {} directories here:", lst.len() - 2).to_string(),
    };
    for dir_name in lst {
        if dir_name == "." || dir_name == "" { continue; }
        result.push_str("\n");
        result.push_str(&dir_name[2..]);
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "hello" {
        println!("{}", cmd_hello());
    } else if args[1] == "sum" {
        println!("{}", cmd_sum(args));
    } else if args[1] == "dirs" {
        println!("{}", cmd_dirs());
    } else {
        println!("undefined action for args[0]={}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd_hello, cmd_sum, cmd_dirs};

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
        assert_eq!(
            cmd_dirs(), "There are 4 directories here:\ntarget\nsrc\n.git\n.idea"
        );
    }
}

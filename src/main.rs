use std::env;

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
            },
        }
    }
    return format!("Sum of given numbers is {}", res);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "hello" {
        println!("{}", cmd_hello());
    } else if args[1] == "sum" {
        println!("{}", cmd_sum(args));
    } else {
        println!("undefined action for args[0]={}", args[1]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{cmd_hello, cmd_sum};

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
}

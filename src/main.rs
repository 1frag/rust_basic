use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "hello" {
        println!("Hello, world!");
    }

    else if args[1] == "sum" {
        let mut res = 0;
        for i in 2..args.len() {
            match args[i].parse::<i32>() {
                Ok(n) => res += n,
                Err(e) => {
                    println!("{} couldn't be parsed ({})", args[i], e);
                },
            }
        }
        println!("Sum of given numbers is {}", res);
    }

    else {
        println!("undefined action for args[0]={}", args[1]);
    }
}

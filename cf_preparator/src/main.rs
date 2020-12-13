extern crate regex;

use std::fs;
use std::env;
use std::cmp::Ordering;
use regex::Regex;

fn fetch_problem_url() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        return Some(args[1].to_string());
    }
    println!("Incorrect usage");
    return None;
}

fn fetch_libs(contents: &str) -> String {
    let mut result = "".to_string();
    for (_, line) in contents.lines().into_iter().enumerate() {
        if line == "" { break; }
        result = format!("{}{}\n", result, line);
    }
    return result;
}

fn fetch_block(content: &str, mark: String) -> Option<String> {
    let at_eq = |ind: usize, other: &str| -> bool {
        return content[ind..ind+1].cmp(other) == Ordering::Equal;
    };

    let opt_ind = content.find(&mark);
    if opt_ind.is_none() { return None; }
    let mut ind = opt_ind.unwrap() as usize;

    while !at_eq(ind, "\n") { ind -= 1; }
    ind += 1;
    let start = ind.clone();

    let (mut brackets, mut was) = (0, false);
    while !was || (brackets != 0) {
        if at_eq(ind, "{") { brackets += 1; was = true; }
        if at_eq(ind, "}") { brackets -= 1; }
        ind += 1;
    }
    return Some(content[start..ind].to_string());
}

fn fetch_deps(content: &str, target: String) -> String {
    let re = Regex::new(r"use crate::\{(.+)}").unwrap();
    let cap = re.captures(&target);
    if cap.is_none() { return "".to_string() }

    return cap
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .split(", ")
        .map(|dep| fetch_block(content, dep.to_string()).unwrap())
        .fold("".to_string(), |i, j| format!("{}{}\n\n", i, j));
}

fn make_main(target: &str) -> String {
    let re = Regex::new(r"pub mod (\w+) \{").unwrap();
    let cap = re.captures(&target);
    if cap.is_none() { panic!("mod not found") }
    let name = cap.unwrap().get(1).unwrap().as_str();

    format!(r"
fn main() {}
    {}::solve();
{}", "{", name, "}")
}

fn get_ident(url: String) -> String {
    url
        .split("/")
        .enumerate()
        .filter(|(i, _)| i > &4)
        .fold("".to_string(), |i, j| format!("{}{}", i, j.1))
}

fn main() {
    let problem_url_opt = fetch_problem_url();
    if problem_url_opt.is_none() { return; }
    let problem_url = problem_url_opt.unwrap();
    let ident = get_ident(problem_url.clone());

    let filename = "codeforces/src/main.rs";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mod_code_opt = fetch_block(&contents, problem_url.clone());
    if mod_code_opt.is_none() {
        return println!("Couldn't parse main function");
    }
    let mod_code = mod_code_opt.unwrap();

    let code = format!(
        "{}\n{}{}\n{}\n",
        fetch_libs(&contents),
        fetch_deps(&contents, mod_code.clone()),
        mod_code.clone(),
        make_main(&mod_code)
    );

    fs::write(
        format!("cf_preparator/gen/{}.rs", ident),
        code
    ).expect("Something went wrong writing the file");
}

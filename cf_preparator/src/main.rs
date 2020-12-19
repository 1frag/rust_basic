extern crate regex;

use colored::*;
use regex::Regex;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::process::Command;

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
        if line == "" {
            break;
        }
        result = format!("{}{}\n", result, line);
    }
    return result;
}

fn fetch_block(content: &str, mark: String) -> Option<String> {
    let at_eq = |ind: usize, other: &str| -> bool {
        return content[ind..ind + 1].cmp(other) == Ordering::Equal;
    };

    let opt_ind = content.find(&mark);
    if opt_ind.is_none() {
        return None;
    }
    let mut ind = opt_ind.unwrap() as usize;

    while !at_eq(ind, "\n") {
        ind -= 1;
    }
    ind += 1;
    let start = ind.clone();

    let (mut brackets, mut was) = (0, false);
    while !was || (brackets != 0) {
        if at_eq(ind, "{") {
            brackets += 1;
            was = true;
        }
        if at_eq(ind, "}") {
            brackets -= 1;
        }
        ind += 1;
    }
    return Some(content[start..ind].to_string());
}

fn fetch_deps(content: &str, target: String) -> String {
    let re = Regex::new(r"use crate::\{(.+)}").unwrap();
    let cap = re.captures(&target);
    if cap.is_none() {
        return "".to_string();
    }

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
    if cap.is_none() {
        panic!("mod not found")
    }
    let name = cap.unwrap().get(1).unwrap().as_str();

    format!(
        r"
fn main() {}
    {}::solve();
{}",
        "{", name, "}"
    )
}

fn compile(source_path: &str, compile_path: &str) {
    let cmd = format!("rustc {} -o {}", source_path, compile_path);
    println!(
        "{:?}",
        Command::new("sh")
            .args(&["-c", &cmd])
            .output()
            .unwrap()
            .stdout
    );
}

fn copy_to_clipboard(t: &str) {
    Command::new("bash")
        .args(&["-c", &format!("echo -n '{}' | xsel -ib", t)])
        .output()
        .unwrap()
        .stdout;
}

fn main() {
    let problem_ident_opt = fetch_problem_url();
    if problem_ident_opt.is_none() {
        return;
    }
    let problem_ident = problem_ident_opt.unwrap();

    let filename = "codeforces/src/main.rs";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mod_code_opt = fetch_block(&contents, problem_ident.clone());
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

    let source_path = format!("cf_preparator/gen/{}.rs", problem_ident);
    let compile_path = format!("cf_preparator/gen/{}", problem_ident);
    fs::write(&source_path, code).expect("Something went wrong writing the file");
    compile(&source_path, &compile_path);

    let mut invite = format!("cd cf_preparator/gen/ && ./{} > out.txt &", problem_ident);
    invite.push_str("& echo -ne \"\\033[0;33m\" && cat out.txt && echo -ne \"\\033[0m\" && cd -");

    copy_to_clipboard(&invite);
    println!("Check:\n{}", (&invite).blue().bold());
}

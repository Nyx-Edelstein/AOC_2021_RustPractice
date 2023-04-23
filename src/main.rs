use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod solutions;

fn main()
{
    solve("06", solutions::day06::solution_b, false);
}

fn solve(day: &str, solution_func: fn(&str) -> String, test : bool)
{
    let input = get_input(day, test);
    let result = solution_func(input.as_str());
    println!("{}", result);
}

fn get_input(day: &str, test: bool) -> String
{
    let filename = if test {
        format!("./input_test/{}.txt", day)
    } else {
        format!("./input/{}.txt", day)
    };
    let path = Path::new(&filename);
    let mut file = File::open(path).unwrap_or_else(|_| panic!("cannot find file at {}", filename));
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap_or_else(|_| panic!("cannot read file at {}", filename));
    
    input
}
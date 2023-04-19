use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod solutions;

fn main() {
    let input = get_input("04", false);
    //let result = solutions::day04::solution_a(&input);
    let result = solutions::day04::solution_b(&input);

    println!("{}", result);
}

fn get_input(day: &str, test: bool) -> String
{
    let filename = if test {
        format!("./inputs/{}_test.txt", day)
    } else {
        format!("./inputs/{}.txt", day)
    };
    let path = Path::new(&filename);
    let mut file = File::open(path).unwrap_or_else(|_| panic!("cannot find file at {}", filename));
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap_or_else(|_| panic!("cannot read file at {}", filename));
    
    input
}
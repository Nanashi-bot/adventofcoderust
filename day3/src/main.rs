use std::fs;
use regex::Regex;

fn main() {
    let file_path = "/home/aditya/codes/learningrust/day3/data/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");
    let mut sum: i32 = 0;
    for line in contents.lines() {

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for captures in re.captures_iter(line) {


            let num1_str = captures.get(1).unwrap().as_str();
            let num2_str = captures.get(2).unwrap().as_str();
            let num1 = num1_str.parse::<i32>().unwrap();
            let num2 = num2_str.parse::<i32>().unwrap();

            println!("Found mul({},{})", num1, num2);
            sum += num1 * num2;
        }
    }
    println!("Final sum: {}", sum);
}

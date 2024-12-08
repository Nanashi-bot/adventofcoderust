use std::fs;

fn main() {

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let file_path = "/home/aditya/codes/learningrust/day1/data/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    for line in contents.lines() {
        let numbers: Vec<&str> = line.trim().split("   ").collect();
        if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()){
            list1.push(num1);
            list2.push(num2);
        }
    }
    list1.sort();
    list2.sort();
    //println!("List 1: {:?}", &list1[..10]);
    //println!("List 2: {:?}", list2);

    let result: Vec<i32> = list1.iter().zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    let sum: i32 = result.iter().sum();

    println!("The sum is: {}", sum);

}

use std::fs;
use std::collections::HashMap;

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

    // PART 1 SOLUTION
    let result: Vec<i32> = list1.iter().zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .collect();

    let sum: i32 = result.iter().sum();

    println!("The answer for part 1 is: {}", sum);

    // PART 2 SOLUTION
    let mut count_map = HashMap::new();
    for &elem in &list2 {
        *count_map.entry(elem).or_insert(0) += 1;
    }

    let mut sim_sum: i32 = 0;

    for &elem in &list1 {
        let count = count_map.get(&elem).unwrap_or(&0);
        sim_sum += elem * count;
        //println!("Element {} appears {} times in list2", elem, count);
    }
    println!("The answer for part 2 is: {}", sim_sum);

}

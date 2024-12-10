use std::fs;

fn is_sequence_safe(nums: &[i32]) -> bool {
    let increasing = nums[1] > nums[0];
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff.abs() > 3 || diff == 0 || (increasing && diff < 0) || (!increasing && diff > 0) {
            return false;
        }
    }
    true
}

fn main() {

    let file_path = "/home/aditya/codes/learningrust/day2/data/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut safe_count : i32 = 0;

    // PART 1 SOLUTION
//    for line in contents.lines() {
//        //println!("\n{}", line);
//        let nums: Vec<i32> = line.trim().split_whitespace().map(|s| s.parse::<i32>().expect("Failed to parse number"))
//                                                          .collect();
//
//        let mut is_safe = true;
//        let increasing = nums[1] > nums[0];
//        for i in 0..nums.len() - 1 {
//            let diff = nums[i+1] - nums[i];
//            if diff.abs() > 3 || diff == 0 || (increasing && diff < 0) || (!increasing && diff > 0) {
//                is_safe = false;
//                break
//            }
//        }
//        if is_safe {
//            safe_count += 1;
//        }
//    }
//    println!("Total number of cases: {}", contents.lines().count());
//    println!("Number of safe cases: {}", safe_count)


    // PART 2 SOLUTION
    for line in contents.lines() {
        let nums: Vec<i32> = line.trim().split_whitespace().map(|s| s.parse::<i32>().expect("Failed to parse number"))
                                                          .collect();
        let mut is_safe = true;
        let increasing = nums[1] > nums[0];
        for i in 0..nums.len() - 1 {
            let diff = nums[i+1] - nums[i];
            if diff.abs() > 3 || diff == 0 || (increasing && diff < 0) || (!increasing && diff > 0) {
                is_safe = false;
                break;
                }
        }
        if !is_safe {
            for i in 0..nums.len() {
                // Create a new sequence with the i-th element removed
                let mut modified_nums = nums.to_vec();
                modified_nums.remove(i);

                // Check if the modified sequence is safe
                if is_sequence_safe(&modified_nums) {
                    println!("Modified is correct by removing {} from {}", nums[i], line);
                    is_safe = true;
                    break;
                }
            }
        }
        if is_safe {
            safe_count += 1;
        }
    }
    println!("Total number of cases: {}", contents.lines().count());
    println!("Number of safe cases: {}", safe_count)
}

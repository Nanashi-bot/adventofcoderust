use std::fs;

fn main() {

    let file_path = "/home/aditya/codes/learningrust/day4/data/input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    let mut grid = Vec::new();

    for line in contents.lines() {
        let row: Vec<char> = line
            .split_whitespace()
            .map(|s| s.chars().collect::<Vec<char>>())
            .flat_map(|v| v)
            .collect();
        grid.push(row);
    }

    let mut sum: i32 = 0;
//    let word: Vec<char> = "XMAS".chars().collect();

//    for row in &grid {
//        println!("{:?}", row);
//    }

//    let directions = [
//        (-1,0),(-1,1),(-1,-1),(1,0),(1,1),(1,-1),(0,0),(0,1),(0,-1)
//    ];
//
//    for i in 0..grid.len(){
//        for j in 0..grid[i].len(){
//            for (dx,dy) in &directions {
//                if word_search(&grid, i, j, &word, *dx, *dy) {
//                    println!("Found starting at ({}, {}) in direction ({}, {})", i, j, dx,dy);
//                    sum += 1;
//                }
//            }
//        }
//    }
//    println!("Total times found: {}", sum);

    // PART 2
    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            let mut c: i32 = 0;
            if grid[i][j] == 'A'{
                if grid[i-1][j+1] == 'M' && grid[i+1][j-1] == 'S' { c2 += 1; }
                if grid[i+1][j+1] == 'M' && grid[i-1][j-1] == 'S' { c2 += 1; }
                if grid[i+1][j-1] == 'M' && grid[i-1][j+1] == 'S' { c2 += 1; }
                if grid[i-1][j-1] == 'M' && grid[i+1][j+1] == 'S' { c2 += 1; }
                if c > 1{
                    println!("A found at ({}, {})", i, j);
                    sum += 1;
                }
            }
        }
    }
    println!("{}", sum);
}

fn word_search(grid: &Vec<Vec<char>>, x: usize, y:usize, word: &Vec<char<>>, dx: isize, dy: isize) -> bool {
    let mut cur_x = x as isize;
    let mut cur_y = y as isize;

    for k in 0..word.len() {
        if cur_x < 0 || cur_x >= grid.len() as isize || cur_y < 0 || cur_y >= grid[0].len() as isize {
            return false;
        }

        if grid[cur_x as usize][cur_y as usize] != word[k] {
            return false;
        }
        cur_x += dx;
        cur_y += dy;
    }
    true
}

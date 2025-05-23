use std::cmp::max;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn solve_it() -> i64 {
    let path = Path::new("./files/0067_triangle.txt");
    let mut file = match File::open(&path) {
        Err(why) => { panic!("Couldn't open {}: {}", path.display(), why.to_string()) },
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(why) = file.read_to_string(&mut contents) { panic!("Couldn't read {}: {}", path.display(), why.to_string()) }

    let mut triangle = Vec::<Vec<i64>>::new();

    for line in contents.lines() {
        let mut row = Vec::<i64>::new();
        for num_str in line.split_whitespace() {
            row.push(num_str.parse::<i64>().unwrap());
        }
        triangle.push(row);
    }

    // top down
    for row in 1..triangle.len() {
        for col in 0..triangle[row].len() {
            let mut max_parent: i64 = 0;

            if col > 0 {
                max_parent = triangle[row-1][col-1]
            }

            if col < triangle[row-1].len() {
                max_parent =  max(max_parent, triangle[row-1][col])
            }

            triangle[row][col] += max_parent;
        }
    }

    *triangle[triangle.len()-1].iter().max().unwrap()
}
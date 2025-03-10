use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::util::triangular_number;

pub fn euler42() -> i64 {
    let path = Path::new("./files/0042_words.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    let mut num_triangle_words = 0;

    let words = contents.split(',');    // this iterator doesn't need to be mut, and the compiler suggests I remove it if I have it. Why? Aren't we mutating the iterator when we iterate over it?

    let mut triangle_iter = (1..).map(|n| triangular_number(n));
    let mut largest_triangle_number = 0;
    let mut triangular_numbers = HashSet::<i64>::new();

    for raw_word in words {
        let word = raw_word.trim_matches('"');
        let word_sum = word.chars().map(|c| (c as i64 - 'A' as i64) + 1).sum::<i64>(); // without the ::<i64> on the sum, this has all kinds of errors. Why?

        while word_sum > largest_triangle_number {
            let next_triangular_number = triangle_iter.next().unwrap();
            triangular_numbers.insert(next_triangular_number);
            largest_triangle_number = next_triangular_number;
        }

        if triangular_numbers.contains(&word_sum) {
            num_triangle_words += 1;
        }
    }

    num_triangle_words
}
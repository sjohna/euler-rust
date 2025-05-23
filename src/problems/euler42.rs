use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::util::{integer, next_list};

pub fn euler42() -> i64 {
    let path = Path::new("./files/0042_words.txt");
    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(why) = file.read_to_string(&mut contents) { panic!("Couldn't read {}: {}", path.display(), why) }

    let mut num_triangle_words = 0;

    let words = contents.split(',');

    let mut triangular_numbers = next_list::NextList::from_iter(Box::new((1..).map(integer::triangular_number)));

    for raw_word in words {
        let word = raw_word.trim_matches('"');
        let word_sum = word.chars().map(|c| (c as i64 - 'A' as i64) + 1).sum::<i64>();

        triangular_numbers.advance_to(word_sum);

        if triangular_numbers.contains_key(word_sum) {
            num_triangle_words += 1;
        }
    }

    num_triangle_words
}
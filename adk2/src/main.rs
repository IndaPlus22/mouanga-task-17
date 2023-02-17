// powered by the kattis io template

use std::io;
use std::env;
use std::fs;
const INPUT_MODE_FILE: bool = false;

fn main() {
    
    if !INPUT_MODE_FILE {
        let input = io::stdin();
        let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    } else {
        let input = fs::read_to_string(ordlista.txt).expect("");
    }



}

// get levenstein distance between two words.
fn compare_words(a: String, b: String) -> usize {
let a_len = a.len();
let b_len = b.len();

let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(a_len);

for i in 0..a_len + 1 {
    // edit distance between nothing and a substring is equal to the len of the substring
    matrix[i][0] = i;
}

for i in 0..b_len + 1 {
    // edit distance between nothing and a substring is equal to the len of the substring
    matrix[0][i] = i;
}


/*
now follow two rules:
if the current letter in a is not equal to the current letter in b
the levenstein distance is one plus the minimum of [i-1][j-1], [i][j-1] and [i-1][j], where [i][j] is our current position
in the matrix
and if they are equal then the levenstein distance is simply equal to [i-1][j-1]
 */


}
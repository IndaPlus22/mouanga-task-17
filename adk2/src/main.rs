// powered by the kattis io template
// and indians on youtube https://www.youtube.com/watch?v=b6AGUjqIPsA ; i swear on honor and conscience (??) that i didnt use any of his code
// (not that it matters because its not even in rust)

use std::io;
//use std::env;
//use std::fs;
//const INPUT_MODE_FILE: bool = false;

fn main() {
    /* 
    if !INPUT_MODE_FILE {
        let input = io::stdin();
        let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    } else {
        let input = fs::read_to_string(ordlista.txt).expect("");
    }
*/

let mut input1 = String::new();
let mut input2 = String::new();
let mut dictionary: Vec<String> = vec![];

let stdin1 = io::stdin();
stdin1.read_line(&mut input1).expect("err");
let stdin2 = io::stdin();
stdin2.read_line(&mut input2).expect("err");
println!("{}", compare_words(
    String::from(input1.trim()), 
    String::from(input2.trim())
));

}

fn count_chars(string: &str) -> usize {
    string.chars().count()
}

// minimum of three
fn mot(a: usize, b: usize, c: usize) -> usize {
    if a < b {
        if a < c { return a; }
        else { return c; }
    } else {
        if b < c { return b; }
        else { return c;}
    }
    panic!("logic error");
}

// get levenstein distance between two words.
fn compare_words(a: String, b: String) -> usize {
let a_len = count_chars(&a);
let b_len = count_chars(&b);

let a_chars: Vec<char> = a.chars().collect();
let b_chars: Vec<char> = b.chars().collect();

let mut matrix: Vec<Vec<usize>> = vec![vec![0; b_len + 1]; a_len + 1];

for i in 0..(a_len + 1) {
    // edit distance between nothing and a substring is equal to the len of the substring
    matrix[i][0] = i;
}

for i in 0..(b_len + 1) {
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
for i in 1..(a_len + 1) {
    for j in 1..(b_len + 1) {
        // we're dealing with the same character
        if a_chars[i-1] == b_chars[j-1] {
            matrix[i][j] = matrix[i-1][j-1];
        } else {
        // they are different
            matrix[i][j] = 1usize + mot(matrix[i-1][j-1],
                matrix[i][j-1],
                matrix[i-1][j]
            );
        }
    }
}

return matrix[a_len][b_len];


}
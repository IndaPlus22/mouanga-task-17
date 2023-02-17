// powered by the kattis io template
// and indians on youtube https://www.youtube.com/watch?v=b6AGUjqIPsA ; i swear on honor and conscience (??) that i didnt use any of his code
// (not that it matters because its not even in rust)

use std::io;
use std::io::prelude::*;
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

let mut input = String::new();
let mut dictionary: Vec<String> = vec![];
let mut misspelled: Vec<String> = vec![];

let stdin = io::stdin();
// make the dictionary
let mut lines = stdin
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    // and get one line at a time,
loop {
    let next_line = lines.next().unwrap();
    if next_line != "#" {
        dictionary.push(next_line);
    } else {
        break;
    }
}

// make the dictionary of misspelled words
loop {
    let next_line = lines.next().unwrap();
    misspelled.push(next_line);
}

// now we just compare every felstavat ord to every word in the dictionary... this wont take O(inf) runtime at all lol
for word in misspelled {
    let mut suggestions: Vec<String> = vec![];
    let mut best_distance: usize = 40;
    for suggestion in dictionary {
        let cmp_words = compare_words(word, suggestion);
        if cmp_words < best_distance {
            // reset the list; we found something better
            best_distance = cmp_words;
            suggestions = vec![];
        }

        if cmp_words == best_distance {
            // add the word to the list of suggestions
            suggestions.push(suggestion);
        }
    }
    println!("{} ({}) {}", word, best_distance, strvec_to_str(suggestions))
}


/*let stdin1 = io::stdin();
stdin1.read_line(&mut input1).expect("err");
let stdin2 = io::stdin();
stdin2.read_line(&mut input2).expect("err");
println!("{}", compare_words(
    String::from(input1.trim()), 
    String::from(input2.trim())
));
*/

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

fn strvec_to_str(vec: Vec<String>) -> String {
    let mut string = String::new();
    for element in vec {
        string.push_str(&element);
    }
    return string;
}
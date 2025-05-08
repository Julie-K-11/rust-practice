use std::io::{self, BufRead};

/*staircase
 * https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn stair_case(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    stair_case(n);
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* diagonalDifference
 * https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut sum_of_primary_ = 0;
    let mut sum_of_secondary = 0;

    for i in 0..n {
        sum_of_primary_ += arr[i][i];
        sum_of_secondary += arr[i][n - 1 - i];
    }

    (sum_of_primary_ - sum_of_secondary).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonal_difference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

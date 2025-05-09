use std::io::{self, BufRead};

/* miniMaxSum
 * https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn mini_max_sum(arr: &[i32]) {
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();
    let min_sum = total_sum - *arr.iter().max().unwrap() as i64;
    let max_sum = total_sum - *arr.iter().min().unwrap() as i64;

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    mini_max_sum(&arr);
}

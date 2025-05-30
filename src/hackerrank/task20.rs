use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* sockMerchant
 * https://www.hackerrank.com/challenges/sock-merchant/problem?isFullScreen=true
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sock_merchant(n: i32, ar: &[i32]) -> i32 {
    let mut sorted_ar = ar.to_vec();
    sorted_ar.sort();

    let mut pairs = 0;
    let mut i = 0;

    while i < sorted_ar.len() - 1 {
        if sorted_ar[i] == sorted_ar[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}

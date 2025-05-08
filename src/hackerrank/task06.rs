use std::io::{self, BufRead};

/*plusMinus
 * https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plus_minus(arr: &[i32]) {
    let len = arr.len() as f64;
    let (mut pos, mut neg, mut zero) = (0, 0, 0);

    for &num in arr {
        if num > 0 {
            pos += 1;
        } else if num < 0 {
            neg += 1;
        } else {
            zero += 1;
        }
    }

    println!("{:.6}", pos as f64 / len);
    println!("{:.6}", neg as f64 / len);
    println!("{:.6}", zero as f64 / len);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}

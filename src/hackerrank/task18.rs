use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* migratoryBirds
 * https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut bird_type = 0;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            bird_type = i as i32;
        }
    }

    bird_type
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratory_birds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

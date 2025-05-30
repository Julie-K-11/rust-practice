use std::io::{self, BufRead};

/* bonAppetit
 * https://www.hackerrank.com/challenges/bon-appetit/problem?isFullScreen=true
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let total_cost: i32 = bill.iter().sum();
    let anna_share = (total_cost - bill[k as usize]) / 2;

    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bon_appetit(&bill, k, b);
}

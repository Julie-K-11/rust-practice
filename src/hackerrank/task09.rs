use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/* timeConversion
 * https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn time_conversion(s: &str) -> String {
    let am_pm = &s[8..];
    let mut hour: i32 = s[0..2].parse().unwrap();

    if am_pm == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else {
        if hour != 12 {
            hour += 12;
        }
    }

    format!("{:02}{}", hour, &s[2..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

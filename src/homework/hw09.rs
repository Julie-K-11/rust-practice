/*написати код:
    - який виконує зсув рядка
*/

fn rotate(s: &str, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }

    let n = ((n % len) + len) % len;
    let split_index = (len - n) as usize;
    let (left, right) = s.split_at(split_index);

    let mut result = right.to_string();
    result.push_str(left);
    result
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s, *n),
                exp.to_string()
            )
        );
}

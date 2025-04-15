/*написати код:
    - який вираховує greatest common divisor
    - (https://en.wikipedia.org/wiki/Greatest_common_divisor)
*/

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[test]
fn test() {
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];

    for ((a, b), exp) in data.iter() {
        let result = gcd(*a, *b);
        println!("{a} and {b} = {result}, expected {exp}");
        assert_eq!(*exp, gcd(*a, *b));
    }
}
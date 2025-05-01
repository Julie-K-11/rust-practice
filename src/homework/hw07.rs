/*написати код:
    - міняє регістр з верхнього на нижній
    - міняє регістр з нижнього на верхнього
*/

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_lowercase() => c.to_uppercase().to_string(),
            c if c.is_uppercase() => c.to_lowercase().to_string(),
            _ => c.to_string(),
        })
        .collect()
}

#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}

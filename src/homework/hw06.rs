/*написати код:
    - який малює ялинку в консолі
    - ялинка має виглядати як в доданому файлі
    - єдиний параметр для конфігурації - кількість трикутників
    - в коді бажано використовувати iterators
*/

#[test]
fn main() {
    let num = 6;

    for i in 0..num {
        for j in 0..= i {
            let spaces = num - j - 1;
            let symbols = 2 * j + 1;

            print!("{}", " ".repeat(spaces));
            print!("{}", "*".repeat(symbols));
            println!();
        }
    }
}



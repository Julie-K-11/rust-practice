/*написати код:
    -Умова задачі в доданому файлі (5 клас, але треба написати програму:)
    - Написати функцію для знаходження змінних m, u, x, a, s, l, o, n
    - вивести результат у форматі:
      muxa
    x    a
      ----
      slon
    - скільки рішень має задача?
    - ВМІСТ ФАЙЛУ : у цьому записі треба замінити літери поставити счисла від 1 до 8. Однаковим літерам відповідають однаківі числа
*/

fn permute(mut items: Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    generate(&mut items, 0, &mut result);
    result
}

fn generate(items: &mut Vec<u32>, i: usize, result: &mut Vec<Vec<u32>>) {
    if i == items.len() {
        result.push(items.clone());
        return;
    }

    for j in i..items.len() {
        items.swap(i, j);
        generate(items, i + 1, result);
        items.swap(i, j); // backtrack
    }
}

#[test]
fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    let perms = permute(digits.to_vec());
    for p in perms {
        let m = p[0];
        let u = p[1];
        let x = p[2];
        let a = p[3];
        let s = p[4];
        let l = p[5];
        let o = p[6];
        let n = p[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            println!("{m}{u}{x}{a}");
            println!("×  {a}");
            println!("----");
            println!("{slon:>4}\n");
            count += 1;
        }
    }

    println!("Кількість рішень: {}", count);
}



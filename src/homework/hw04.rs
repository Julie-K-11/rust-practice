/*
Фактично, написати программу яка:
    - малює ромб в консолі
    - ромб має виглядати як указано в доданому файлі
    - розмір ромб має задаватися двома константами на початку коду
    - для написання потрібно використовувати цикли for
    - в коді дозволяється використовувати print! один раз
    - в коді дозволяється використовувати println! один раз
*/

#[test]
fn diamond() {
    const H: u32 = 7;
    const W: u32 = 7;

    let center_y = H / 2;

    for y in 0..H {
        for x in 0..W {
            let dist_y = (y as i32 - center_y as i32).abs() as u32;
            let left_bound = dist_y;
            let right_bound = W - 1 - dist_y;

            let to_show = x >= left_bound && x <= right_bound;
            let sym = if to_show { "*" } else { " " };
            print!("{}", sym);
        }
        println!();
    }
}

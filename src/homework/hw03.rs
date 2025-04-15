/*
Фактично, написати програму яка:
    - малює конверт в консолі
    - конверт має виглядати як указано в доданому файлі
    - розмір конверта має задаватися двома константами на початку коду
    - для написання потрібно використовувати цикли for
    - в коді дозволяється використовувати print! один раз
    - в коді дозволяється використовувати println! один раз
 */

#[test]
fn envelope(){
    const W: u32 = 28;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W{
            let k:f32 = W as f32 / (H - 1) as f32;
            let is_hor = y == 0 || y == H - 1;
            let is_ver = x == 0 || x == W - 1;
            let is_diag1 = x == (y as f32 * k) as u32;
            let is_diag2 = (y as f32 * k) as u32 == W - x - 1;
            let to_show = is_hor || is_ver || is_diag1 || is_diag2;

            let sym= if to_show {"*"} else {" "};
            print!("{}", sym);
        }
        println!();
    }
}

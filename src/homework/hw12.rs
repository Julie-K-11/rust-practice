/*написати код:
    - Написати функцію яка рахує мінімальну кількість переносу грузу щоб на всіх кораблях був однаковий груз fn count_permutation(shipments: &Vec<u32>) -> usize
    - Чи завжди можливо всі кораблі забезпечити однаковою кількість грузу?
    - як буде виглядати сігнатура в іншому випадку?
    - написати функцію генерації Vec<32> які можуть бути розподілені однаково. fn gen_shipments(n: usize) -> Vec<u32>
*/
use rand::Rng;

fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1;
    }

    let avg = total / n;
    let moves: u32 = shipments
        .iter()
        .filter(|&&cargo| cargo > avg)
        .map(|&cargo| cargo - avg)
        .sum();

    moves as i32
}


fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::rng();
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.random_range(1..=10)).collect();
    let total: u32 = shipments.iter().sum();
    let rem = total % n as u32;

    if rem != 0 {
        shipments[0] += n as u32 - rem;
    }

    shipments
}

#[test]
fn main() {
    let tests = [
        (vec![1, 1, 1, 1, 6], 4),
        (vec![9, 3, 7, 2, 9], 7),
        (vec![8, 2, 2, 4, 4], 4),
        (vec![1, 2, 3], 1),
        (vec![46, 57, 76, 55, 94], -1),
    ];

    println!("Тестові вектори:");
    for (v, expected) in tests {
        let real = count_permutation(&v);
        println!("{v:?} - очікується: {expected:>2}, отримано: {real:>2}");
    }

    println!("\nРандомний вектор:");
    let vec_rnd = gen_shipments(5);
    let moves = count_permutation(&vec_rnd);
    if moves == -1 {
        println!("{vec_rnd:?} - неможливо забезпечити одинаковий розподіл", );
    } else {
        println!("{vec_rnd:?} - мінімальна кількість переміщень = {moves}");
    }
}
/*написати код:
    - генерує рандомний вектор довжиною 20 зі значеннями [10..99]    fn gen_random_vector(n: usize) -> Vec<i32>
    - знаходить минімальну пару у Vec<i32>: fn min_adjacent_sum(data: &[i32]) -> ???
    - Написати функцію, яка виводить в консоль в зрозумілому вигляді (див доданий файл)
*/

use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_sum)
}

fn print_result(data: &[i32], min_index: usize, min_sum: i32) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:   ");
    for &value in data {
        print!("{:>3},", value);
    }
    println!();

    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index {
            print!("{:>width$}", "\\_ _/", width = 7);
            break;
        } else {
            print!("{:>4}", "");
        }
    }
    println!();

    println!(
        "min adjacent sum = {} + {} = {} at indexes: {}, {}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
    println!();
}

#[test]
fn main() {
    for _ in 0..4 {
        let data = gen_random_vector(20);
        let (index, min_sum) = min_adjacent_sum(&data);
        print_result(&data, index, min_sum);
    }
}

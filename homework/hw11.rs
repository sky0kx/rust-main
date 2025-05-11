use rand::Rng;

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

pub fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1])
}

pub fn print_data_with_min_pair(data: &[i32]) {
    // 1. Вивести індекси
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // 2. Вивести самі значення
    print!("data:  [");
    for (i, val) in data.iter().enumerate() {
        if i < data.len() - 1 {
            print!("{}, ", val);
        } else {
            print!("{}", val);
        }
    }
    println!("]");

    // 3. Підкреслити мінімальну пару
    let (min_i, a, b) = min_adjacent_sum(data);
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_i {
            print!("\\__ ");
        } else if i == min_i + 1 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();

    // 4. Показати результат
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        a,
        b,
        a + b,
        min_i,
        min_i + 1
    );
}

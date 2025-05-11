use rand::Rng;

/// Функція підрахунку мінімальної кількості переміщень для вирівнювання грузу
pub fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None; // Неможливо вирівняти
    }

    let avg = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &ship in shipments {
        balance += ship as i32 - avg as i32;
        moves += balance.abs() as usize;
    }

    Some(moves)
}

/// Генерує вектор грузів, які можна розподілити рівномірно між кораблями
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = rand::thread_rng().gen_range(10..30);
    let mut shipments = vec![avg; n];

    // Робимо невеликі варіації, зберігаючи загальну суму сталою
    for _ in 0..(n / 2) {
        let i = rand::thread_rng().gen_range(0..n);
        let j = rand::thread_rng().gen_range(0..n);
        let delta = rand::thread_rng().gen_range(1..=avg.min(5));

        if shipments[i] >= delta {
            shipments[i] -= delta;
            shipments[j] += delta;
        }
    }

    shipments
}

/// Пояснення результату в консолі
pub fn explain(shipments: Vec<u32>) {
    println!("Shipments: {:?}", shipments);
    let total: u32 = shipments.iter().sum();
    println!("Total = {}", total);

    match count_permutation(&shipments) {
        Some(moves) => {
            let avg = total / shipments.len() as u32;
            println!("Average = {}", avg);
            println!("Minimum transfers needed = {}", moves);
        }
        None => {
            println!("Cannot evenly distribute the cargo among ships.");
        }
    }
}

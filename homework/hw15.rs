use itertools::Itertools;

fn get_digits(m: u8, u: u8, x: u8, a: u8) -> u32 {
    (m as u32) * 1000 + (u as u32) * 100 + (x as u32) * 10 + (a as u32)
}

fn get_result(s: u8, l: u8, o: u8, n: u8) -> u32 {
    (s as u32) * 1000 + (l as u32) * 100 + (o as u32) * 10 + (n as u32)
}

pub fn solve() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let mut solutions = vec![];

    for perm in (1u8..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = get_digits(m, u, x, a);
        let slon = get_result(s, l, o, n);

        if muxa * (a as u32) == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }
    }

    solutions
}

fn main() {
    let solutions = solve();
    for (m, u, x, a, s, l, o, n) in &solutions {
        println!("  {}{}{}{}", m, u, x, a);
        println!("x       {}", a);
        println!("--------");
        println!("  {}{}{}{}", s, l, o, n);
        println!();
    }

    println!("Total solutions: {}", solutions.len());
}

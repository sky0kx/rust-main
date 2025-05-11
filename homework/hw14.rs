pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::new();

    // Додаємо "0" до початку першої половини
    for code in &prev {
        result.push(format!("0{}", code));
    }

    // Додаємо "1" до початку другої половини (в зворотному порядку)
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!["".to_string()]),
            (1, vec!["0", "1"]
                .into_iter().map(String::from).collect()),
            (2, vec!["00", "01", "11", "10"]
                .into_iter().map(String::from).collect()),
            (3, vec!["000", "001", "011", "010", 
                     "110", "111", "101", "100"]
                .into_iter().map(String::from).collect()),
            (4, vec!["0000", "0001", "0011", "0010",
                     "0110", "0111", "0101", "0100",
                     "1100", "1101", "1111", "1110",
                     "1010", "1011", "1001", "1000"]
                .into_iter().map(String::from).collect()),
        ];

        test_data
            .iter()
            .for_each(|(n, out)| assert_eq!(gray(*n), *out));
    }
}

fn main() {
    println!("Hello, world!");
}

fn powers_of_two(n: u8) -> Vec<u128> {
    let mut exponentiation_of_2_base = Vec::new();

    for number in 0..n + 1 {
        exponentiation_of_2_base.push(2u128.pow(number as u32));
    }
    return exponentiation_of_2_base;
}


#[cfg(test)]
mod tests {
    use super::powers_of_two;

    fn dotest(n: u8, expected: &[u128]) {
        let actual = powers_of_two(n);
        assert_eq!(actual, expected, "With n = {n}\nExpected {expected:?}\nBut got {actual:?}")
    }

    #[test]
    fn fixed_for_0() {
        dotest(0, &[1]);
    }

    #[test]
    fn fixed_for_1() {
        dotest(1, &[1, 2]);
    }

    #[test]
    fn fixed_tests() {
        dotest(0, &[1]);
        dotest(1, &[1, 2]);
        dotest(4, &[1, 2, 4, 8, 16]);
    }
}






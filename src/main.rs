fn main() {
    println!("Hello, world!");
}

fn reverse_seq(n: u32) -> Vec<u32> {

    let mut number = n;
    let mut numbers = Vec::new();
    loop {
        numbers.push(number);
        number -= 1;
        if number <= 0 {
            break;
        }
    }
    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}



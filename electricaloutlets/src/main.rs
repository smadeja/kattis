use std::io;

fn main() {
    let mut cases = String::new();
    io::stdin().read_line(&mut cases).unwrap();

    let cases: u32 = cases.trim().parse().unwrap();

    for _ in 0..cases {
        let mut power_strips = String::new();
        io::stdin().read_line(&mut power_strips).unwrap();

        let mut power_strips: Vec<u32> = power_strips
            .split(" ")
            .map(|power_strip| power_strip.trim().parse().unwrap())
            .collect();

        power_strips.remove(0);

        println!("{}", appliances(power_strips));
    }
}

fn appliances(power_strips: Vec<u32>) -> u32 {
    power_strips
        .iter()
        .fold(0, |acc, power_strip| acc + power_strip - 1)
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_appliance_count() {
        assert_eq!(appliances(vec![2, 3, 4]), 7);
        assert_eq!(appliances(vec![4, 4, 4, 4, 4, 4, 4, 4, 4, 4]), 31);
        assert_eq!(appliances(vec![10, 10, 10, 10]), 37);
    }
}

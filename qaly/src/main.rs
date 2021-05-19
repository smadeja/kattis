use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();

    let mut periods: Vec<(f64, f64)> = Vec::new();

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let period: Vec<f64> = line.split(" ").map(|x| x.trim().parse().unwrap()).collect();
        periods.push((period[0], period[1]));
    }

    println!("{}", qaly(periods));
}

fn qaly(periods: Vec<(f64, f64)>) -> f64 {
    periods
        .iter()
        .fold(0.0, |acc, (quality, years)| acc + (*quality * *years))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_qaly() {
        assert_eq!(
            qaly(vec![
                (1.0, 12.0),
                (0.7, 5.2),
                (0.9, 10.7),
                (0.5, 20.4),
                (0.2, 30.0)
            ]),
            41.47
        );
    }
}

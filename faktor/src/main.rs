use std::io::stdin;

fn bribe_count(article_count: u32, impact_factor: u32) -> u32 {
    article_count * (impact_factor - 1) + 1
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();

    let article_count: u32 = input[0].parse().unwrap();
    let impact_factor: u32 = input[1].parse().unwrap();

    println!("{}", bribe_count(article_count, impact_factor));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_bribe_count_for_sample_1() {
        assert_eq!(bribe_count(38, 24), 875);
    }

    #[test]
    fn computes_bribe_count_for_sample_2() {
        assert_eq!(bribe_count(1, 100), 100);
    }

    #[test]
    fn computes_bribe_count_for_sample_3() {
        assert_eq!(bribe_count(10, 10), 91);
    }
}

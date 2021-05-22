use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let line: Vec<i32> = line.split(" ").map(|x| x.trim().parse().unwrap()).collect();

    println!("{}", r2(line[0], line[1]));
}

fn r2(r1: i32, s: i32) -> i32 {
    (2 * s) - r1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_r2() {
        assert_eq!(r2(11, 15), 19);
        assert_eq!(r2(4, 3), 2);
    }
}

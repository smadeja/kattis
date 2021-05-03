use std::io;
use std::convert::TryInto;

fn main() {
    let addend_count = parse_line();
    let mut sum = 0;

    for _i in 0..addend_count {
        let line = parse_line();

        let number = line / 10;
        let power = (line % 10).try_into().unwrap();

        sum += number.pow(power);
    }

    println!("{}", sum);
}

fn parse_line() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    line.trim().parse().unwrap()
}

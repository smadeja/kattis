use std::io;

fn main() {
    let mut iterations = String::new();

    io::stdin().read_line(&mut iterations).unwrap();
    let iterations: u32 = iterations.trim().parse().unwrap();

    println!("{}", points(iterations));
}

fn points(iterations: u32) -> u32 {
    if iterations > 0 {
        let previous = (points(iterations - 1) as f64).sqrt() as u32;

        ((previous * 2) - 1).pow(2)
    } else {
        4
    }
}

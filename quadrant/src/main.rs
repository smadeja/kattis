use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();

    io::stdin().read_line(&mut x).unwrap();
    io::stdin().read_line(&mut y).unwrap();

    let x: i32 = x.trim().parse().unwrap();
    let y: i32 = y.trim().parse().unwrap();

    println!("{}", quadrant(x, y));
}

fn quadrant(x: i32, y: i32) -> i32 {
    if y > 0 {
        if x > 0 {
            1
        } else {
            2
        }
    } else {
        if x < 0 {
            3
        } else {
            4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_first_quadrant() {
        assert_eq!(quadrant(2, 3), 1);
    }

    #[test]
    fn computes_second_quadrant() {
        assert_eq!(quadrant(-2, 3), 2);
    }

    #[test]
    fn computes_third_quadrant() {
        assert_eq!(quadrant(-2, -3), 3);
    }

    #[test]
    fn computes_fourth_quadrant() {
        assert_eq!(quadrant(2, -3), 4);
    }
}

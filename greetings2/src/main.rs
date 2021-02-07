use std::convert::TryInto;
use std::io;

fn e_count(text: &str) -> u32 {
    let mut count = 0;

    for letter in text.chars() {
        if letter == 'e' {
            count += 1;
        }
    }

    count
}

fn greeting_reply(greeting: &str) -> String {
    let e_count: usize = (e_count(greeting) * 2).try_into().unwrap();
    let es = "e".repeat(e_count);

    format!("h{}y", es)
}

fn main() {
    let mut greeting = String::new();
    io::stdin().read_line(&mut greeting).unwrap();
    let greeting = greeting.trim();

    println!("{}", greeting_reply(&greeting));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_es_for_sample_1() {
        assert_eq!(e_count("hey"), 1);
    }

    #[test]
    fn counts_es_for_sample_2() {
        assert_eq!(e_count("heeeeey"), 5);
    }

    #[test]
    fn generates_reply_for_sample_1() {
        assert_eq!(greeting_reply("hey"), "heey");
    }

    #[test]
    fn generates_reply_for_sample_2() {
        assert_eq!(greeting_reply("heeeeey"), "heeeeeeeeeey");
    }
}

use std::io;

fn main() {
    let stdin = io::stdin();

    let mut monthly_quota = String::new();
    let mut month_count = String::new();

    stdin.read_line(&mut monthly_quota).unwrap();
    stdin.read_line(&mut month_count).unwrap();

    let monthly_quota: i32 = monthly_quota.trim().parse().unwrap();
    let month_count: u32 = month_count.trim().parse().unwrap();

    let mut transferred_quota = 0;

    for _ in 0..month_count {
        let mut used_quota = String::new();
        stdin.read_line(&mut used_quota).unwrap();
        let used_quota: i32 = used_quota.trim().parse().unwrap();

        transferred_quota += monthly_quota - used_quota;
    }

    println!("{}", monthly_quota + transferred_quota)
}

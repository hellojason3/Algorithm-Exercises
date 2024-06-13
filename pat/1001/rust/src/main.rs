fn divide_match(num: u32) -> u32 {
    match num {
        1 => 1,
        n if n % 2 == 0 => n / 2,
        n if n % 2 == 1 => (3 * n + 1) / 2,
        _ => {
            panic!("unreachable")
        }
    }
}
fn main() {
    let mut num_str = String::new();
    std::io::stdin()
        .read_line(&mut num_str)
        .expect("cannot read line");
    let num: u32 = num_str.trim().parse().expect("not a number");
    let mut count = 0u32;
    let mut n = num;
    loop {
        if n == 1 {
            break;
        }
        count += 1;
        n = divide_match(n);
    }
    println!("{}", count);
}

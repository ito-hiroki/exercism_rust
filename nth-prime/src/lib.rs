pub fn nth(n: u32) -> u32 {
    (1..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let sqrt_num = (n as f64).sqrt() as u32 + 1;
    for i in (3..sqrt_num).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

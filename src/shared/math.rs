pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

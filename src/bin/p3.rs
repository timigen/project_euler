pub fn solution() {
    println!("\nproblem 3 _______________________________________");

    _loop();
}

pub fn _loop() {
    let mut factor: u64 = 1;
    let target: u64 = 600851475143;

    loop {
        factor += 1;
        if factor >= target {
            break;
        }

        if is_prime(factor) {
            println!("\n\tis_prime: {}", factor);
        }
    }
}

pub fn is_even(n: u64) -> bool {
    return n % 2 == 0;
}

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n < 2 || is_even(n) {
        return false;
    }

    let ceiling = (n as f64).sqrt() as u64 + 1;

    for x in 2..ceiling {
        if x % 2 == 0 {
            continue;
        }

        if n % x == 0 {
            return false;
        }
    }

    return true;
}

pub fn solution() {
    println!("\nproblem 2 _______________________________________");

    _loop();
}

pub fn _loop() {
    let mut x_one: u64 = 1;
    let mut x_two: u64 = 2;
    let mut x_next: u64;
    let mut _sum: u64 = 2;

    loop {
        x_next = x_one + x_two;
        x_one = x_two;
        x_two = x_next;

        if x_next > 4000000 {
            break;
        }

        if x_next % 2 == 0 {
            _sum += x_next;
        }
    }
    println!("\n\twhile sum\t: {}", _sum);
}

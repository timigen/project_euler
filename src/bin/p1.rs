pub fn solution() {
    println!("\nproblem 1 _______________________________________");

    for_loop();
    filter();
}

pub fn for_loop() {
    let mut sum:i32 = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    println!("\n\tfor\t: {}", sum);
}

pub fn filter() {
    let sum : i32 = (0..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("\tfilter\t: {}", sum);
}

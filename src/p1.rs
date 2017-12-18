
    pub fn naive() {
        let mut sum:i32 = 0;

        for x in 0..1000 {
            if x % 3 == 0 || x % 5 == 0 {
                sum += x;
            }
        }

        println!("sum {}", sum);
    }

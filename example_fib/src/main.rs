fn main() {
    pub fn fibonacci(n: u8) -> f64 {
        if n < 0 {
            panic!("{} is negative!", n);
        } else if n == 0 {
            panic!("zero is not a right argument to fibonacci()!");
        } else if n == 1 {
            return 1.0;
        }

        let mut sum = 0.0;
        let mut last = 0.0;
        let mut curr = 1.0;
        for _i in 1..n {
            sum = last + curr;
            last = curr;
            curr = sum;
        }
        sum
    }

    let fuzzer = sidefuzz::SideFuzz::new(1, |message: &[u8]| {
        if message[0] == 0u8 {
            return Err(());
        }
        sidefuzz::black_box(fibonacci(message[0]));
        Ok(())
    });

    fuzzer.run();
}

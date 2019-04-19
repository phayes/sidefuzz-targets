use subtle::ConstantTimeEq;

fn main() {
    println!("This example target should be constant time, so it will never exit.");

    let fuzzer = sidefuzz::SideFuzz::new(24, |message: &[u8]| {
        sidefuzz::black_box(message.ct_eq(b"CONSTANT TIME COMPARISON"));
        Ok(())
    });

    fuzzer.run();
}

use subtle::ConstantTimeEq;

fn main() {
    println!("This example target should be constant time, so it will never exit.");

    let fuzzer = sidefuzz::SideFuzz::new(24, |m1: &[u8]| {
        let mut m2 = m1.to_vec();
        if !m2.is_empty() {
            m2[0] ^= m1[0];
        }

        sidefuzz::black_box(m2.ct_eq(m1));
        Ok(())
    });

    fuzzer.run();
}

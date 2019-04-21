
fn main() {
    fn not_constant_time_comparison(m1: &[u8]) -> bool {
        let mut m2 = m1.to_vec();
        if !m2.is_empty() {
            m2[0] ^= m1[0];
        }

        m1 == &m2[..]
    }

    let fuzzer = sidefuzz::SideFuzz::new(28, |message: &[u8]| {
        sidefuzz::black_box(not_constant_time_comparison(message));
        Ok(())
    });

    fuzzer.run();
}

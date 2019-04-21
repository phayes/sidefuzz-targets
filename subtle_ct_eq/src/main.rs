use subtle::ConstantTimeEq;

fn main() {
    let fuzzer = sidefuzz::SideFuzz::new(24, #[inline(never)]
    |message: &[u8]| {
        sidefuzz::black_box(message.ct_eq(b"CONSTANT TIME COMPARISON"));
    });

    fuzzer.run();
}

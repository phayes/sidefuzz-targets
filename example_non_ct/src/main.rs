
fn main() {
    fn not_constant_time_comparison(message: &[u8]) -> bool {
        message == b"NOT CONSTANT TIME COMPARISON"
    }

    let fuzzer = sidefuzz::SideFuzz::new(28, #[inline(never)] |message: &[u8]| {
        sidefuzz::black_box(not_constant_time_comparison(message));
    });

    fuzzer.run();
}

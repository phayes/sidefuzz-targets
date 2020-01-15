#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(32);
    sidefuzz::black_box(fuzz_blake3(input));
}

pub fn fuzz_blake3(n: &[u8]) -> [u8; 32] {
    let mut hasher = reference_impl::Hasher::new();
    hasher.update(n);

    let mut output = [0; 32];
    hasher.finalize(&mut output);

    output
}

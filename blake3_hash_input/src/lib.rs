#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(32);
    sidefuzz::black_box(fuzz_blake3(input));
}

pub fn fuzz_blake3(n: &[u8]) -> blake3::Hash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(n);
    hasher.finalize()
}

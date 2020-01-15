#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(32);
    sidefuzz::black_box(fuzz_blake3(input));
}

pub fn fuzz_blake3(n: &[u8]) -> blake3::Hash {
    let key = [0; 32];
    blake3::keyed_hash(&key, n)
}

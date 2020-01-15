use std::convert::TryInto;

#[no_mangle]
pub extern "C" fn fuzz() {
    let key = sidefuzz::fetch_input(32);
    sidefuzz::black_box(fuzz_blake3(key));
}

pub fn fuzz_blake3(key: &[u8]) -> blake3::Hash {
    let input = [0; 32];
    let key: &[u8; 32] = key.try_into().expect("slice with incorrect length");
    blake3::keyed_hash(key, &input)
}

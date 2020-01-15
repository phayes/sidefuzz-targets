#[no_mangle]
pub extern "C" fn fuzz() {
    let key_material = sidefuzz::fetch_input(8);
    sidefuzz::black_box(fuzz_blake3(key_material));
}

pub fn fuzz_blake3(key_material: &[u8]) -> [u8; 8] {
    let mut output = [0; 8];
    blake3::derive_key("sidefuzz", &key_material, &mut output);

    output
}

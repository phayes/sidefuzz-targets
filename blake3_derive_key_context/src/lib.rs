use std::str;

#[no_mangle]
pub extern "C" fn fuzz() {
    // We need to fetch a low number of bytes here since we will panic if they are not utf8
    // It's not ideal, since it limits the search space, but what we have to work with
    // since derive_key takes a &str as a context.
    let context = sidefuzz::fetch_input(2);

    let context = str::from_utf8(context).unwrap();
    sidefuzz::black_box(fuzz_blake3(context));
}

pub fn fuzz_blake3(context: &str) -> [u8; 8] {
    let key_material = [0; 8];
    let mut output = [0; 8];
    blake3::derive_key(context, &key_material, &mut output);

    output
}

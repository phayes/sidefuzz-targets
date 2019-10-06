extern crate orion;

use orion::util::secure_cmp;
use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;


#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(CHACHA_KEYSIZE as i32);
    sidefuzz::black_box(cmp(&input));
}

fn cmp(input: &[u8]) -> bool {
    secure_cmp(&[0u8; CHACHA_KEYSIZE], input).is_ok()
}
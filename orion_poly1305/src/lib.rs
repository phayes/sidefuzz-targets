extern crate orion;

use orion::hazardous::mac::poly1305::*;

#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(89);
    let sk = OneTimeKey::from_slice(&input[..32]).unwrap();

    sidefuzz::black_box(poly1305(&sk, &input[32..]).unwrap());
}
extern crate orion;

use orion::hazardous::stream::chacha20::{SecretKey, CHACHA_KEYSIZE};

#[no_mangle]
pub extern "C" fn fuzz() {
    let input = sidefuzz::fetch_input(CHACHA_KEYSIZE as i32);
    let zero_sk = SecretKey::from_slice(&[0u8; CHACHA_KEYSIZE]).unwrap();
    let gen_sk = SecretKey::from_slice(input).unwrap();

    sidefuzz::black_box(secret_key_pt_eq(&zero_sk, &gen_sk));
}

fn secret_key_pt_eq(zero_sk: &SecretKey, gen_sk: &SecretKey) -> bool {   
    zero_sk == gen_sk
}